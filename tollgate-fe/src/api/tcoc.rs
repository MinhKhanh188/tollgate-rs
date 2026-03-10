// tollgate-fe\src\api\tcoc.rs
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Clone, Copy)]
pub struct TcocHeader {
    pub length: u32,
    pub command_id: u32,
    pub request_id: u32,
    pub session_id: u32,
}

impl TcocHeader {
    pub const SIZE: usize = 16;
}

pub async fn write_string<W: AsyncWriteExt + Unpin>(
    writer: &mut W,
    s: &str,
    len: usize,
) -> io::Result<()> {
    let mut buf = vec![0u8; len];
    let bytes = s.as_bytes();
    let copy_len = bytes.len().min(len);
    buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
    writer.write_all(&buf).await
}

pub async fn read_string<R: AsyncReadExt + Unpin>(
    reader: &mut R,
    len: usize,
) -> io::Result<String> {
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf).await?;
    let end = buf.iter().position(|&b| b == 0).unwrap_or(len);
    let s = String::from_utf8_lossy(&buf[..end]).into_owned();
    Ok(s.trim().to_string())
}

#[derive(Debug, Clone)]
pub struct ConnectRequest {
    pub username: String, // 10 bytes
    pub password: String, // 10 bytes
    pub station: u32,     // 4 bytes
    pub timeout: u32,     // 4 bytes
}

impl ConnectRequest {
    pub async fn read<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            username: read_string(reader, 10).await?,
            password: read_string(reader, 10).await?,
            station: reader.read_u32_le().await?,
            timeout: reader.read_u32_le().await?,
        })
    }
    pub async fn write<W: AsyncWriteExt + Unpin>(&self, writer: &mut W) -> io::Result<()> {
        write_string(writer, &self.username, 10).await?;
        write_string(writer, &self.password, 10).await?;
        writer.write_u32_le(self.station).await?;
        writer.write_u32_le(self.timeout).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ConnectResponse {
    pub status: u32,
}

impl ConnectResponse {
    pub async fn read<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            status: reader.read_u32_le().await?,
        })
    }
    pub async fn write<W: AsyncWriteExt + Unpin>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32_le(self.status).await
    }
}

#[derive(Debug, Clone)]
pub struct CheckinRequest {
    pub etag: String,       // 24
    pub station: u32,       // 4
    pub lane: u32,          // 4
    pub plate: String,      // 10
    pub tid: String,        // 24
    pub hash_value: String, // 16
}

impl CheckinRequest {
    pub async fn read<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            etag: read_string(reader, 24).await?,
            station: reader.read_u32_le().await?,
            lane: reader.read_u32_le().await?,
            plate: read_string(reader, 10).await?,
            tid: read_string(reader, 24).await?,
            hash_value: read_string(reader, 16).await?,
        })
    }
    pub async fn write<W: AsyncWriteExt + Unpin>(&self, writer: &mut W) -> io::Result<()> {
        write_string(writer, &self.etag, 24).await?;
        writer.write_u32_le(self.station).await?;
        writer.write_u32_le(self.lane).await?;
        write_string(writer, &self.plate, 10).await?;
        write_string(writer, &self.tid, 24).await?;
        write_string(writer, &self.hash_value, 16).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CheckinResponse {
    pub status: u32,
    pub etag: String, // 24
    pub station: u32,
    pub lane: u32,
    pub ticket_id: u32,
    pub ticket_type: u32,
    pub price: u32,
    pub vehicle_type: u32,
    pub plate: String, // 10
    pub plate_type: u32,
    pub price_ticket_type: u32,
}

impl CheckinResponse {
    pub async fn read<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            status: reader.read_u32_le().await?,
            etag: read_string(reader, 24).await?,
            station: reader.read_u32_le().await?,
            lane: reader.read_u32_le().await?,
            ticket_id: reader.read_u32_le().await?,
            ticket_type: reader.read_u32_le().await?,
            price: reader.read_u32_le().await?,
            vehicle_type: reader.read_u32_le().await?,
            plate: read_string(reader, 10).await?,
            plate_type: reader.read_u32_le().await?,
            price_ticket_type: reader.read_u32_le().await?,
        })
    }
    pub async fn write<W: AsyncWriteExt + Unpin>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32_le(self.status).await?;
        write_string(writer, &self.etag, 24).await?;
        writer.write_u32_le(self.station).await?;
        writer.write_u32_le(self.lane).await?;
        writer.write_u32_le(self.ticket_id).await?;
        writer.write_u32_le(self.ticket_type).await?;
        writer.write_u32_le(self.price).await?;
        writer.write_u32_le(self.vehicle_type).await?;
        write_string(writer, &self.plate, 10).await?;
        writer.write_u32_le(self.plate_type).await?;
        writer.write_u32_le(self.price_ticket_type).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CommitRequest {
    pub etag: String, // 24
    pub station: u32,
    pub lane: u32,
    pub ticket_id: u32,
    pub status: u32,
    pub plate: String, // 10
    pub image_count: u32,
    pub vehicle_length: u32,
    pub transaction_amount: u32,
    pub weight: u32,
    pub reason_id: u32,
}

impl CommitRequest {
    pub async fn read<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            etag: read_string(reader, 24).await?,
            station: reader.read_u32_le().await?,
            lane: reader.read_u32_le().await?,
            ticket_id: reader.read_u32_le().await?,
            status: reader.read_u32_le().await?,
            plate: read_string(reader, 10).await?,
            image_count: reader.read_u32_le().await?,
            vehicle_length: reader.read_u32_le().await?,
            transaction_amount: reader.read_u32_le().await?,
            weight: reader.read_u32_le().await?,
            reason_id: reader.read_u32_le().await?,
        })
    }
    pub async fn write<W: AsyncWriteExt + Unpin>(&self, writer: &mut W) -> io::Result<()> {
        write_string(writer, &self.etag, 24).await?;
        writer.write_u32_le(self.station).await?;
        writer.write_u32_le(self.lane).await?;
        writer.write_u32_le(self.ticket_id).await?;
        writer.write_u32_le(self.status).await?;
        write_string(writer, &self.plate, 10).await?;
        writer.write_u32_le(self.image_count).await?;
        writer.write_u32_le(self.vehicle_length).await?;
        writer.write_u32_le(self.transaction_amount).await?;
        writer.write_u32_le(self.weight).await?;
        writer.write_u32_le(self.reason_id).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CommitResponse {
    pub status: u32,
}

impl CommitResponse {
    pub async fn read<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            status: reader.read_u32_le().await?,
        })
    }
    pub async fn write<W: AsyncWriteExt + Unpin>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_u32_le(self.status).await
    }
}

pub async fn write_header<W: AsyncWriteExt + Unpin>(
    writer: &mut W,
    header: &TcocHeader,
) -> io::Result<()> {
    writer.write_u32_le(header.length).await?;
    writer.write_u32_le(header.command_id).await?;
    writer.write_u32_le(header.request_id).await?;
    writer.write_u32_le(header.session_id).await?;
    Ok(())
}

pub async fn read_header<R: AsyncReadExt + Unpin>(reader: &mut R) -> io::Result<TcocHeader> {
    let length = reader.read_u32_le().await?;
    let command_id = reader.read_u32_le().await?;
    let request_id = reader.read_u32_le().await?;
    let session_id = reader.read_u32_le().await?;
    Ok(TcocHeader {
        length,
        command_id,
        request_id,
        session_id,
    })
}
