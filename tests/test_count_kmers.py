# -*- coding: utf-8 -*-
#
#   This file is part of the kcounter package, available at:
#   https://github.com/apcamargo/kcounter
#
#   kcounter is free software: you can redistribute it and/or modify
#   it under the terms of the GNU General Public License as published by
#   the Free Software Foundation, either version 3 of the License, or
#   (at your option) any later version.
#
#   This program is distributed in the hope that it will be useful,
#   but WITHOUT ANY WARRANTY; without even the implied warranty of
#   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#   GNU General Public License for more details.
#
#   You should have received a copy of the GNU General Public License
#   along with this program. If not, see <https://www.gnu.org/licenses/>.
#
#   Contact: antoniop.camargo@gmail.com


import kcounter
import pytest


def test_kcounter_absolute():
    assert kcounter.count_kmers('TCATTCGATT', 3) == {
        'GAT': 1.0,
        'TTC': 1.0,
        'CGA': 1.0,
        'CAT': 1.0,
        'TCA': 1.0,
        'ATT': 2.0,
        'TCG': 1.0,
    }


def test_kcounter_relative():
    assert kcounter.count_kmers('TCATTCGATT', 3, relative_frequencies=True) == {
        'TCG': 0.125,
        'CGA': 0.125,
        'TCA': 0.125,
        'TTC': 0.125,
        'ATT': 0.25,
        'GAT': 0.125,
        'CAT': 0.125,
    }


def test_kcounter_canonical():
    assert kcounter.count_kmers('TCATTCGATT', 3, canonical_kmers=True) == {
        'ATG': 1.0,
        'AAT': 2.0,
        'TCA': 1.0,
        'GAA': 1.0,
        'CGA': 2.0,
        'ATC': 1.0,
    }


def test_kcounter_large_k():
    assert kcounter.count_kmers('TCATTCGATT', 11) == {}


def test_kcounter_negative_k():
    with pytest.raises(ValueError):
        kcounter.count_kmers('TCATTCGATT', -1)


def test_kcounter_zero_k():
    with pytest.raises(ValueError):
        kcounter.count_kmers('TCATTCGATT', 0)


def test_kcounter_non_dna_characters():
    kcounter.count_kmers('CAGNACATGGNTCACATYCT', 4) == {
        'TCAC': 1.0,
        'ACAT': 2.0,
        'CATG': 1.0,
        'ATGG': 1.0,
        'CACA': 1.0,
    }


def test_kcounter_only_non_dna_characters():
    kcounter.count_kmers('NNYNRRNNNKNNWW', 4) == {}


def test_kcounter_lowercase_characters():
    assert kcounter.count_kmers('TCAtTCgATT', 3) == {
        'GAT': 1.0,
        'TTC': 1.0,
        'CGA': 1.0,
        'CAT': 1.0,
        'TCA': 1.0,
        'ATT': 2.0,
        'TCG': 1.0,
    }