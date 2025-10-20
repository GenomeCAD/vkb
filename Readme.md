# Variant Knowledge Base

VKB are split in two base:
- exploded: Many table for variant, annotation, sample information and disease information
- unified: One unique iceberg table with many repetition partitioned

## Pipeline

```mermaid
flowchart LR
    Exploded[(Exploded)]
    Unified[(Unified)]
    gvcf>gvcf]
    vcf>vcf]
    tsv>tsv]
    phenopacket>phenopacket]
    json>json]
    vkb_convert([vkb_convert])
    vkb_exploded2unified([vkb_exploded2unified])
	web[[web interface]]

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
erDiagram
    Variant {
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

	Coverage {
		string chromosome
		int position
		int an
	}

    Gnomad {
        string chromosome
        int position
        string reference
        string alternate
        float gnomad_af
        int gnomad_an
        int gnomad_ac
    }

	ClinVar {
        string chromosome
        int position
        string reference
        string alternate
		string clinical_signifiance
	}

    VepSnpeff {
        string chromosome
        int position
        string reference
        string alternate
        string impact
        string effect
    }

    AnnotSv {
        string chromosome
        int start
        int end
        string second_chromosome
        int second_start
        int second_end
        string impact
        string effect
    }

    Genotyping {
        string chromosome
        int start
        int end
        string reference
        string alternate
        string sample_name
        enum genotype
        enum inheritance
    }

    Sample {
        string sample_name
        bool affected
		string preindication
        string hpos
		string karyotypic_sex
    }

    Variant ||--o{ Coverage: chromosome_start
    Variant ||--o{ Gnomad: chromosome_start_reference_alternative
    Variant ||--o{ VepSnpeff: chromosome_start_reference_alternative
    Variant ||--o{ ClinVar: chromosome_start_reference_alternative
	Variant ||--o{ AnnotSv: chromosome_start_reference_alternative
    Variant ||--o{ Genotyping: chromosome_start_reference_alternative
    Genotyping ||--o{ Sample: sample_name
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
| string | vepsnpeff_impact     | No        | `AnnotSv`    |
| string | vepsnpeff_effect     | No        | `AnnotSv`    |
| bool   | affected             | No        | `Sample`     |
| string | preindication        | Yes       | `Sample`     |
| string | hpos                 | Yes       | `Sample`     |
| string | karyotypic_sex       | Yes       | `Sample`     |
| string | sample_name          | Yes       | `Genotyping` |
| string | inheritance          | Yes       | `Genotyping` |

[Details](doc/unified_schema.md)

## vkb_convert

## vkb_exploded2unified
