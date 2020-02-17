use needletail::{kmer::CanonicalKmers, Sequence};
use pyo3::{exceptions, prelude::*, wrap_pyfunction};
use std::collections::HashMap;
use String;

/// count_kmers(sequence, k, relative_frequencies=False, canonical_kmers=False)
/// --
///
/// Counts the k-mers of a DNA sequence.
///
/// Parameters
/// ----------
/// sequence : str
///    DNA sequence string from which k-mers will be counted.
/// k : int
///    Length of the k-mers. Must be a positive number.
/// relative_frequencies : bool
///    Return relative k-mer frequencies. Default is False.
/// canonical_kmers : bool
///    Count the canonical representation of k-mers. Default is False.
///
/// Returns
/// -------
/// dict
///    Dictionary where the keys are k-mers and the values are their counts or relative frequencies.
///
/// Notes
/// -----
/// K-mers containing characters other than ATCG are ignored.
///
/// Examples
/// --------
/// >>> kcounter.count_kmers('AAACTTTTTT', 3)
/// {'ACT': 1.0, 'AAA': 1.0, 'TTT': 4.0, 'CTT': 1.0, 'AAC': 1.0}
/// >>> kcounter.count_kmers('AAACTTTTTT', 3, relative_frequencies=True)
/// {'AAC': 0.125, 'TTT': 0.5, 'CTT': 0.125, 'ACT': 0.125, 'AAA': 0.125}
/// >>> kcounter.count_kmers('AAACTTTTTT', 3, canonical_kmers=True)
/// {'ACT': 1.0, 'AAA': 5.0, 'AAC': 1.0, 'AAG': 1.0}
#[pyfunction(relative_frequencies = "false", canonical_kmers = "false")]
fn count_kmers(
    sequence: &str,
    k: isize,
    relative_frequencies: bool,
    canonical_kmers: bool,
) -> PyResult<HashMap<String, f64>> {
    if k < 1 {
        Err(exceptions::ValueError::py_err(
            "'k' must be a positive integer.",
        ))
    } else {
        let mut k_counts = HashMap::new();
        let mut k_total = 0.;
        let sequence_uppercase = sequence.to_uppercase();
        let sequence_bytes = sequence_uppercase.as_bytes();
        let sequence_rc = sequence_bytes.reverse_complement();
        let k_iter = CanonicalKmers::new(sequence_bytes, &sequence_rc, k as u8);
        if canonical_kmers {
            for (_position, kmer, _canonical) in k_iter {
                k_total += 1.;
                let kmer = kmer.to_vec();
                let kmer_string = String::from_utf8(kmer).expect("Found invalid UTF-8");
                *k_counts.entry(kmer_string).or_insert(0.) += 1.;
            }
        } else {
            for (_position, kmer, canonical) in k_iter {
                k_total += 1.;
                if canonical {
                    let kmer = kmer.reverse_complement();
                    let kmer_string = String::from_utf8(kmer).expect("Found invalid UTF-8");
                    *k_counts.entry(kmer_string).or_insert(0.) += 1.;
                } else {
                    let kmer = kmer.to_vec();
                    let kmer_string = String::from_utf8(kmer).expect("Found invalid UTF-8");
                    *k_counts.entry(kmer_string).or_insert(0.) += 1.;
                }
            }
        }
        if relative_frequencies {
            let mut k_counts_relative = HashMap::new();
            for (kmer, count) in k_counts.iter() {
                k_counts_relative.insert(kmer.to_string(), count / k_total);
            }
            Ok(k_counts_relative)
        } else {
            Ok(k_counts)
        }
    }
}

#[pymodule]
fn kcounter(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(count_kmers))?;
    Ok(())
}
