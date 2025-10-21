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

        vkb_convert@{shape: st-rect, label: "convert"}
        vkb_exploded2unified@{shape: rect, label: "exploded2unified"}
    end

    subgraph aggregate
        Unified@{shape: cyl, label: "Unified"}
    end

    subgraph public
        BeaconeV2[BeaconeV2]
        web@{shape: notch-rect, label: "web interface"}
    end

    gvcf -->|any time| vkb_convert
    vcf -->|any time| vkb_convert
    tsv -->|any time| vkb_convert
    phenopacket -->|any time| vkb_convert
    json -->|any time| vkb_convert

    vkb_convert --> Exploded

    Exploded -->|select columns| vkb_exploded2unified

    vkb_exploded2unified -->|periodically| Unified

    Unified --> BeaconeV2
    BeaconeV2 --> Unified

    BeaconeV2 --> web
    web --> BeaconeV2
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
			int position
			int an
		}
	}

	namespace Annotation {
		class Gnomad {
			string chromosome
			int position
			string reference
			string alternate
			float gnomad_af
			int gnomad_an
			int gnomad_ac
		}

		class ClinVar {
			string chromosome
			int position
			string reference
			string alternate
			string clinical_signifiance
		}

		class VepSnpeff {
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
			string impact
			string effect
		}
	}

	namespace SampleInformation {
		class Genotyping {
			string chromosome
			int start
			int end
			string reference
			string alternate
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

| type   | field                | mandatory | origin       |
|--------|----------------------|-----------|--------------|
| string | chromosome           | Yes       | `Variant`    |
| int    | start                | Yes       | `Variant`    |
| int    | end                  | Yes       | `Variant`    |
| string | reference            | Yes       | `Variant`    |
| string | alternate            | Yes       | `Variant`    |
| string | variant_type         | Yes       | `Variant`    |
| string | second_chromosome    | No        | `Variant`    |
| int    | second_start         | No        | `Variant`    |
| int    | second_end           | No        | `Variant`    |
| int    | an                   | Yes       | `Coverage`   |
| float  | gnomad_af            | No        | `Gnomad`     |
| string | clinical_signifiance | Yes       | `ClinVar`    |
| string | impact               | Yes       | `VepSnpeff`  |
| string | effect               | Yes       | `VepSnpeff`  |
| string | annotsv_impact       | No        | `AnnotSv`    |
| string | annotsv_effect       | No        | `AnnotSv`    |
| bool   | affected             | No        | `Symptom`    |
| string | preindication        | Yes       | `Symptom`    |
| string | hpos                 | Yes       | `Symptom`    |
| string | karyotypic_sex       | Yes       | `Symptom`    |
| string | sample_name          | Yes       | `Genotyping` |
| string | inheritance          | Yes       | `Genotyping` |

[Details](doc/unified_schema.md)

## Subcommand
### convert

This command are split in two subcommand, one to load data and another to save data.

#### Load

- gvcf: information `Variant`, `Coverage` and `Genotyping`
- vcf: information `Variant`, any `Annotation` table
- tsv: any type of information
- phenopacket: information `Symptom`
- json: any type of information

#### Save

- variant: save loaded information in `Variant`
- coverage: save loaded information in `Coverage`
- annotation: save loaded information in `Annotation` user need to indicate which table are targeted
- symptom: save loaded information in `Symptom`
- genotyping: save loaded information in `Genotyping`

### exploded2unified

This command take information from exploded database and aggregate it in unified database.

User should select which table integrate a parameter indicate which column delete.
Aggregation method are define by subcommand:
- genotyping
