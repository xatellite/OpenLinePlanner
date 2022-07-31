# -*- coding: utf-8 -*-
# snapshottest: v1 - https://goo.gl/zC4yUc
from __future__ import unicode_literals

from snapshottest import GenericRepr, Snapshot


snapshots = Snapshot()

snapshots['CalculateStationTest::test_big 1'] = {
    'station1': {
        'bike': 1749.0,
        'ped': 1152.0,
        'residential': 2901.0,
        'school': 0,
        'total': 2901.0,
        'work': 0
    },
    'station2': {
        'bike': 1915.0,
        'ped': 823.0,
        'residential': 2738.0,
        'school': 0,
        'total': 2738.0,
        'work': 0
    },
    'station3': {
        'bike': 3550.0,
        'ped': 1396.0,
        'residential': 4946.0,
        'school': 0,
        'total': 4946.0,
        'work': 0
    },
    'station4': {
        'bike': 587.0,
        'ped': 8.0,
        'residential': 595.0,
        'school': 0,
        'total': 595.0,
        'work': 0
    },
    'station5': {
        'bike': 1410.0,
        'ped': 249.0,
        'residential': 1659.0,
        'school': 0,
        'total': 1659.0,
        'work': 0
    },
    'station6': {
        'bike': 2793.0,
        'ped': 1993.0,
        'residential': 4786.0,
        'school': 0,
        'total': 4786.0,
        'work': 0
    },
    'station7': {
        'bike': 1231.0,
        'ped': 1680.0,
        'residential': 2911.0,
        'school': 0,
        'total': 2911.0,
        'work': 0
    },
    'station8': {
        'bike': 968.0,
        'ped': 342.0,
        'residential': 796.0,
        'school': GenericRepr('462'),
        'total': 1310.0,
        'work': GenericRepr('52')
    }
}

snapshots['CalculateStationTest::test_small 1'] = {
    'station1': {
        'bike': 1749.0,
        'ped': 1152.0,
        'residential': 2901.0,
        'school': 0,
        'total': 2901.0,
        'work': 0
    },
    'station2': {
        'bike': 3727.0,
        'ped': 1666.0,
        'residential': 5393.0,
        'school': 0,
        'total': 5393.0,
        'work': 0
    }
}
