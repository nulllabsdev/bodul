//! Post-extraction transpose of the feature/spec chart.
//!
//! The architecture extracts the chart **row-major** as `feature_chart.rows`,
//! one row per heading with a `values` vector. Charts can be multi-column
//! (several values per row, one per variant), and columns are positional — they
//! are not DOM elements — so the transpose cannot be expressed with selectors.
//!
//! This rebuilds the data **column-major** under `features`: an array with one
//! entry per column, each a list of `{ label, value }` (the row's label paired
//! with that column's cell). Single-column charts yield a single column.

use serde_json::{Value, json};

/// Transposes `feature_chart.rows` (if present) into a `feature_chart.features`
/// array, replacing `rows` and keeping the chart's other fields (e.g. its
/// heading). A no-op when there is no chart.
pub fn transpose(value: &mut Value) {
    let Some(chart) = value
        .as_object_mut()
        .and_then(|object| object.get_mut("feature_chart"))
        .and_then(Value::as_object_mut)
    else {
        return;
    };
    let Some(rows) = chart.get("rows").and_then(Value::as_array).cloned() else {
        return;
    };

    let columns = rows
        .iter()
        .filter_map(row_values)
        .map(|values| values.len())
        .max()
        .unwrap_or(0);
    let features: Vec<Value> = (0..columns)
        .map(|column| Value::Array(rows.iter().filter_map(|row| cell(row, column)).collect()))
        .collect();

    chart.remove("rows");
    chart.insert("features".to_string(), Value::Array(features));
}

/// The `values` array of a row, if present.
fn row_values(row: &Value) -> Option<&Vec<Value>> {
    row.get("values").and_then(Value::as_array)
}

/// The `{ label, value }` cell for `row` at `column`, or `None` if the row has no
/// label or no value in that column.
fn cell(row: &Value, column: usize) -> Option<Value> {
    let label = row.get("label").and_then(Value::as_str)?;
    let value = row_values(row)?.get(column)?.get("value").and_then(Value::as_str)?;
    Some(json!({ "label": label, "value": value }))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::transpose;

    #[test]
    fn transposes_a_multi_column_chart_and_keeps_other_fields() {
        let mut value = json!({
            "feature_chart": {
                "h2": "Product Specification",
                "rows": [
                    { "label": "CPU", "values": [{ "value": "a1" }, { "value": "a2" }] },
                    { "label": "GPU", "values": [{ "value": "b1" }, { "value": "b2" }] },
                ]
            }
        });

        transpose(&mut value);

        assert_eq!(
            value,
            json!({ "feature_chart": {
                "h2": "Product Specification",
                "features": [
                    [ { "label": "CPU", "value": "a1" }, { "label": "GPU", "value": "b1" } ],
                    [ { "label": "CPU", "value": "a2" }, { "label": "GPU", "value": "b2" } ],
                ]
            } })
        );
    }

    #[test]
    fn transposes_a_single_column_chart() {
        let mut value = json!({ "feature_chart": { "rows": [{ "label": "Model", "values": [{ "value": "X" }] }] } });

        transpose(&mut value);

        assert_eq!(
            value,
            json!({ "feature_chart": { "features": [[{ "label": "Model", "value": "X" }]] } })
        );
    }

    #[test]
    fn is_a_noop_without_a_feature_chart() {
        let mut value = json!({ "product": { "title": "x" } });

        transpose(&mut value);

        assert_eq!(value, json!({ "product": { "title": "x" } }));
    }
}
