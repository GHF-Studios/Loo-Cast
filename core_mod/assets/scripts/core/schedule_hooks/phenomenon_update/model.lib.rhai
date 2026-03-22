// Scale bucket contract:
// 0 = coarsest (largest), 70 = finest (smallest), 35 ~ meter-scale anchor.
private fn phn(id, name, domain_path, kind, scale_min_bucket, scale_max_bucket) {
    #{
        id: id,
        name: name,
        domain_path: domain_path,
        kind: kind,
        scale_min_bucket: scale_min_bucket,
        scale_max_bucket: scale_max_bucket
    }
}

private fn phn_filter_by_scale_bucket(records, bucket_index) {
    let selected = [];
    for record in records {
        if bucket_index >= record["scale_min_bucket"] && bucket_index <= record["scale_max_bucket"] {
            selected.push(record);
        }
    }
    selected
}
