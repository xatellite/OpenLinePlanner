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

snapshots['CalculateStationTest::test_big 2'] = {
    'station1': {
        'bike': 86.88938166183848,
        'ped': 85.94123041170559,
        'residential': 172.83061207354407,
        'school': 0,
        'total': 172.83061207354407,
        'work': 0
    },
    'station2': {
        'bike': 96.52122543418665,
        'ped': 63.08281626228479,
        'residential': 159.60404169647143,
        'school': 0,
        'total': 159.60404169647143,
        'work': 0
    },
    'station3': {
        'bike': 178.83975650106836,
        'ped': 108.23238762143659,
        'residential': 287.0721441225051,
        'school': 0,
        'total': 287.0721441225051,
        'work': 0
    },
    'station4': {
        'bike': 27.31475448995619,
        'ped': 0.4789474720713997,
        'residential': 27.79370196202759,
        'school': 0,
        'total': 27.79370196202759,
        'work': 0
    },
    'station5': {
        'bike': 69.96261555610819,
        'ped': 21.418585684227917,
        'residential': 91.38120124033608,
        'school': 0,
        'total': 91.38120124033608,
        'work': 0
    },
    'station6': {
        'bike': 139.54314546294185,
        'ped': 158.45574305983357,
        'residential': 297.9988885227752,
        'school': 0,
        'total': 297.9988885227752,
        'work': 0
    },
    'station7': {
        'bike': 61.67431579660596,
        'ped': 144.35527949470023,
        'residential': 206.02959529130626,
        'school': 0,
        'total': 206.02959529130626,
        'work': 0
    },
    'station8': {
        'bike': 46.76154701419491,
        'ped': 27.20170255186012,
        'residential': 49.8621825402955,
        'school': 21.647358778403895,
        'total': 73.96324956605504,
        'work': 2.453708247355644
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

snapshots['CalculateStationTest::test_small 2'] = {
    'station1': {
        'bike': 86.88938166183848,
        'ped': 85.94123041170559,
        'residential': 172.83061207354407,
        'school': 0,
        'total': 172.83061207354407,
        'work': 0
    },
    'station2': {
        'bike': 186.0957954209783,
        'ped': 120.80875013736316,
        'residential': 306.9045455583417,
        'school': 0,
        'total': 306.9045455583417,
        'work': 0
    }
}
