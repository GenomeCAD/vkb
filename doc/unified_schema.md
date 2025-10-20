# Unified Database schema

## Genomic partition

- chromosome: identity
- position: truncate($2^{20}$)
- variant_class: identity

## Sample partition

- preindication: identity
- karyotypic_sex: identity
- inheritance: identity
- genotype: identity

## Annotation partition

- clinical_signifiance: identity
- impact: identity
- effect: identity
