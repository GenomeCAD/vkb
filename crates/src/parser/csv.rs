//! Csv parsing

/* std use */

/* crate use */

use itertools::Itertools as _;

/* project use */
use crate::catalog;
use crate::error;

#[derive(std::clone::Clone, derive_builder::Builder)]
#[builder]
pub struct ArrowCsvReader<P>
where
    P: std::convert::AsRef<std::path::Path> + std::clone::Clone,
{
    /// Input has header
    #[builder(default = true)]
    header: bool,

    /// Separator of input file
    #[builder(default = b',')]
    delimiter: u8,

    /// Gziped
    #[builder(default = true)]
    gziped: bool,

    /// Path of input file
    #[builder]
    input_path: P,

    /// Which table parser should match
    #[builder]
    tables: Vec<catalog::Table>,
}

impl<P> ArrowCsvReader<P>
where
    P: std::convert::AsRef<std::path::Path> + std::clone::Clone,
{
    pub fn builder() -> ArrowCsvReaderBuilder<P> {
        ArrowCsvReaderBuilder::default()
    }

    async fn get_reader(
        &self,
    ) -> error::Result<Box<dyn tokio::io::AsyncBufRead + std::marker::Unpin + std::marker::Send>>
    {
        if self.gziped {
            Ok(Box::new(tokio::io::BufReader::new(
                async_compression::tokio::bufread::GzipDecoder::new(tokio::io::BufReader::new(
                    tokio::fs::File::open(self.input_path.as_ref()).await?,
                )),
            )))
        } else {
            Ok(Box::new(tokio::io::BufReader::new(
                tokio::io::BufReader::new(tokio::fs::File::open(self.input_path.as_ref()).await?),
            )))
        }
    }

    pub async fn to_stream(
        self,
    ) -> error::Result<
        impl futures::Stream<Item = Result<arrow_array::RecordBatch, arrow_schema::ArrowError>>,
    > {
        let reader = self.get_reader().await?;

        let columns: Vec<_> = self
            .tables
            .iter()
            .flat_map(catalog::Table::to_name_slice)
            .unique()
            .map(|name| columns()[name].clone())
            .collect();
        log::debug!("Columns {:?}", columns);
        let csv_schema = std::sync::Arc::new(arrow_schema::Schema::new(columns));

        let csv_decoder = arrow_csv::ReaderBuilder::new(csv_schema)
            .with_header(self.header)
            .with_delimiter(self.delimiter)
            .build_decoder();

        Ok(decode_stream(csv_decoder, reader))
    }
}

fn decode_stream<R: tokio::io::AsyncBufRead + std::marker::Unpin>(
    mut decoder: arrow_csv::reader::Decoder,
    mut reader: R,
) -> impl futures::Stream<Item = Result<arrow_array::RecordBatch, arrow_schema::ArrowError>> {
    futures::stream::poll_fn(move |cx| {
        loop {
            let b = match std::task::ready!(std::pin::Pin::new(&mut reader).poll_fill_buf(cx)) {
                Ok(b) => b,
                Err(e) => return futures::task::Poll::Ready(Some(Err(e.into()))),
            };
            let decoded = match decoder.decode(b) {
                // Note: the decoder needs to be called with an empty
                // array to delimit the final record
                Ok(0) => break,
                Ok(decoded) => decoded,
                Err(e) => return futures::task::Poll::Ready(Some(Err(e))),
            };
            std::pin::Pin::new(&mut reader).consume(decoded);
        }

        futures::task::Poll::Ready(decoder.flush().transpose())
    })
}

pub(crate) fn columns() -> &'static std::collections::HashMap<&'static str, arrow_schema::Field> {
    static COLUMNS_MAP: std::sync::OnceLock<std::collections::HashMap<&str, arrow_schema::Field>> =
        std::sync::OnceLock::new();

    COLUMNS_MAP.get_or_init(|| {
        let mut map = std::collections::HashMap::new();

        map.insert(
            "chromosome",
            arrow_schema::Field::new("chromosome", arrow_schema::DataType::Utf8, false),
        );

        map.insert(
            "start",
            arrow_schema::Field::new("start", arrow_schema::DataType::Int64, false),
        );

        map.insert(
            "end",
            arrow_schema::Field::new("end", arrow_schema::DataType::Int64, false),
        );

        map.insert(
            "reference",
            arrow_schema::Field::new("reference", arrow_schema::DataType::Utf8, false),
        );

        map.insert(
            "alternate",
            arrow_schema::Field::new("alternate", arrow_schema::DataType::Utf8, false),
        );

        map.insert(
            "second_chromosome",
            arrow_schema::Field::new("second_chromosome", arrow_schema::DataType::Utf8, true),
        );

        map.insert(
            "second_start",
            arrow_schema::Field::new("second_start", arrow_schema::DataType::Int64, true),
        );

        map.insert(
            "second_end",
            arrow_schema::Field::new("second_end", arrow_schema::DataType::Int64, true),
        );

        map.insert(
            "variant_class",
            arrow_schema::Field::new("variant_class", arrow_schema::DataType::Utf8, false),
        );

        map.insert(
            "an",
            arrow_schema::Field::new("an", arrow_schema::DataType::Int64, true),
        );

        map.insert(
            "gnomad_af",
            arrow_schema::Field::new("gnomad_af", arrow_schema::DataType::Float64, true),
        );

        map.insert(
            "gnomad_an",
            arrow_schema::Field::new("gnomad_an", arrow_schema::DataType::Int64, true),
        );

        map.insert(
            "gnomad_ac",
            arrow_schema::Field::new("gnomad_ac", arrow_schema::DataType::Int64, true),
        );

        map.insert(
            "clinvar_clnsig",
            arrow_schema::Field::new("clinvar_clnsig", arrow_schema::DataType::Utf8, false),
        );

        map.insert(
            "impact",
            arrow_schema::Field::new("impact", arrow_schema::DataType::Utf8, false),
        );

        map.insert(
            "effect",
            arrow_schema::Field::new("effect", arrow_schema::DataType::Utf8, false),
        );

        map.insert(
            "gene_symbol",
            arrow_schema::Field::new("gene_symbol", arrow_schema::DataType::Utf8, false),
        );

        map.insert(
            "transcript_id",
            arrow_schema::Field::new("transcript_id", arrow_schema::DataType::Utf8, false),
        );

        map.insert(
            "canonical",
            arrow_schema::Field::new("canonical", arrow_schema::DataType::Utf8, false),
        );

        map.insert(
            "sample_name",
            arrow_schema::Field::new("sample_name", arrow_schema::DataType::Utf8, true),
        );

        map.insert(
            "genotype",
            arrow_schema::Field::new("genotype", arrow_schema::DataType::Utf8, false),
        );

        map.insert(
            "inheritance",
            arrow_schema::Field::new("inheritance", arrow_schema::DataType::Utf8, false),
        );

        map.insert(
            "ac",
            arrow_schema::Field::new("ac", arrow_schema::DataType::Int64, true),
        );

        map.insert(
            "af",
            arrow_schema::Field::new("af", arrow_schema::DataType::Float64, true),
        );

        map.insert(
            "affected",
            arrow_schema::Field::new("affected", arrow_schema::DataType::Boolean, true),
        );

        map.insert(
            "preindication",
            arrow_schema::Field::new("preindication", arrow_schema::DataType::Utf8, false),
        );

        map.insert(
            "hpos",
            arrow_schema::Field::new("hpos", arrow_schema::DataType::Utf8, false),
        );

        map.insert(
            "karyotypic_sex",
            arrow_schema::Field::new("karyotypic_sex", arrow_schema::DataType::Utf8, false),
        );

        map
    })
}

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */

    /* project use */
}
