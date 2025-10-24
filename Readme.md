# Variant Knowledge Base

VKB are split in two base:
- exploded: Many table for variant, annotation, sample information and disease information
- unified: One unique iceberg table with many repetition partitioned

## Pipeline

```mermaid
flowchart LR
    subgraph not agregate
        Exploded@{shape: cyl, label: "Exploded"}
        gvcf@{shape: docs, label: "gvcf"}
        vcf@{shape: docs, label: "vcf"}
        tsv@{shape: docs, label: "tsv"}
        phenopacket@{shape: docs, label: "phenopacket"}
        json@{shape: docs, label: "json"}
        vkb_aggregate@{shape: rect, label: "aggregate"}
        vkb_convert@{shape: st-rect, label: "convert"}
    end

    subgraph aggregate
        Unified@{shape: cyl, label: "Unified"}
    end

    vkb_csv2unified@{shape: rect, label: "csv2unified"}
    csv@{shape: doc, label: "already aggregate data"}

    subgraph public
        beacon@{shape: rect, label: "beacon server"}
        web@{shape: notch-rect, label: "web interface"}
    end

    gvcf -->|any time| vkb_convert
    vcf -->|any time| vkb_convert
    tsv -->|any time| vkb_convert
    phenopacket -->|any time| vkb_convert
    json -->|any time| vkb_convert

    vkb_convert --> Exploded

    Exploded -->|select columns| vkb_aggregate

    vkb_aggregate -->|periodically| Unified

    csv --> vkb_csv2unified -->|One time| Unified

    Unified --> beacon
    beacon --> Unified

    beacon --> web
    web --> beacon
```


## Exploded schema

```mermaid
---
config:
    class:
      hideEmptyMembersBox: true
---
classDiagram
	direction TB
	namespace StructuralInformation {
		class Variant {
			string chromosome
			int start
			int end
			string reference
			string alternate
			enum variant_type
			string second_chromosome
			int second_start
			int second_end
		}

		class Coverage {
			string chromosome
			int start
			int an
		}
	}

	namespace Annotation {
		class Gnomad {
			string chromosome
			int start
			string reference
			string alternate
			float gnomad_af
			int gnomad_an
			int gnomad_ac
		}

		class ClinVar {
			string chromosome
			int start
			string reference
			string alternate
			string clinvar_clnsig
		}

		class Vep_Snpeff {
			string chromosome
			int position
			string reference
			string alternate
			string impact
			string effect
		}

		class AnnotSv {
			string chromosome
			int start
			int end
			string second_chromosome
			int second_start
			int second_end
			string annotsv_impact
			string annotsv_effect
		}
	}

	namespace SampleInformation {
		class Genotyping {
			string chromosome
			int start
			int end
			string reference
			string alternate
			string second_chromosome
			int second_start
			int second_end
			string sample_name
			enum genotype
			enum inheritance
		}

		class Symptom {
			string sample_name
			bool affected
			string preindication
			string hpos
			string karyotypic_sex
		}
	}

    Variant --> Coverage: chrom_start
    Variant --> Gnomad: chrom_start_ref_alt
    Variant --> VepSnpeff: chrom_start_ref_alt
    Variant --> ClinVar: chrom_start_ref_alt
    Variant --> AnnotSv: chrom_start_ref_alt
    Variant --> Genotyping: chrom_start_ref_alt
    Genotyping --> Symptom: sample_name
```

[Details](doc/exploded_schema.md)

## Unified schema

| type   | field                | mandatory | origin                 |
|--------|----------------------|-----------|------------------------|
| string | chromosome           | Yes       | `Variant`              |
| int    | start                | Yes       | `Variant`              |
| int    | end                  | Yes       | `Variant`              |
| string | reference            | Yes       | `Variant`              |
| string | alternate            | Yes       | `Variant`              |
| enum   | variant_type         | Yes       | `Variant`              |
| string | second_chromosome    | No        | `Variant`              |
| int    | second_start         | No        | `Variant`              |
| int    | second_end           | No        | `Variant`              |
| int    | an                   | Yes       | `Coverage`             |
| float  | gnomad_af            | No        | `Gnomad`               |
| int    | gnomad_an            | No        | `Gnomad`               |
| int    | gnomad_ac            | No        | `Gnomad`               |
| enum   | clinvar_clnsig       | Yes       | `ClinVar`              |
| enum   | impact               | Yes       | `VepSnpeff`/`AnnotSv`  |
| enum   | effect               | Yes       | `VepSnpeff`/`AnnotSv`  |
| string | gene_symbol          | Yes       | `VepSnpeff`/`AnnotSv`  |
| string | transcript_id        | Yes       | `VepSnpeff`/`AnnotSv`  |
| bool   | affected             | No        | `Symptom`              |
| string | preindication        | Yes       | `Symptom`              |
| string | hpos                 | Yes       | `Symptom`              |
| string | karyotypic_sex       | Yes       | `Symptom`              |
| string | sample_name          | No        | `Genotyping`           |
| enum   | genotype             | Yes       | `Genotyping`           |
| enum   | inheritance          | Yes       | `Genotyping`           |
| int    | ac                   | No        | Compute                |
| float  | af                   | No        | Compute                |

[Details](doc/unified_schema.md)

## Features

- `bin`: activate command line interface
- `request`: activate request functionality
- `rest_server`: activate subcommand beacon to launch a server (activate feature `request`)
- `default`: `bin`

## Subcommand

CLI are available only if features `bin` are set.

### convert

This subcommand load data from a classic bioinformatic file and save information in exploded database.

#### Load

- gvcf: information `Variant`, `Coverage` and `Genotyping`
- vcf: information `Variant`, any `Annotation` table
- tsv: any type of information
- phenopacket: information `Symptom`
- json: any type of information

#### Save

- variant: save loaded information in `Variant`
- coverage: save loaded information in `Coverage`
- annotation: save loaded information in table `Annotation`
- symptom: save loaded information in `Symptom`
- genotyping: save loaded information in `Genotyping`

### aggregate

This subcommand take information from exploded database and aggregate it in unified database.

User should select which table integrate a parameter indicate which column delete.
Aggregation method are define by subcommand:
- genotyping

### csv2unified

This subcommand take information from a file and build a unified table with this data.

### beacon

This subcommand start a http Beacon REST server, features `rest_server` are required.
