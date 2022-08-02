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
        'bike': 2266.0,
        'ped': 852.0,
        'residential': 3118.0,
        'school': 0,
        'total': 3118.0,
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
        'bike': 1568.0,
        'ped': 8.0,
        'residential': 1576.0,
        'school': 0,
        'total': 1576.0,
        'work': 0
    },
    'station5': {
        'bike': 1584.0,
        'ped': 249.0,
        'residential': 1833.0,
        'school': 0,
        'total': 1833.0,
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
        'bike': 2204.0,
        'ped': 497.0,
        'residential': 1226.0,
        'school': GenericRepr('1311'),
        'total': 2701.0,
        'work': GenericRepr('164')
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
        'bike': 113.78519231012126,
        'ped': 64.82434670687944,
        'residential': 178.60953901700063,
        'school': 0,
        'total': 178.60953901700063,
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
        'bike': 72.99002552906013,
        'ped': 0.4789474720713997,
        'residential': 73.46897300113154,
        'school': 0,
        'total': 73.46897300113154,
        'work': 0
    },
    'station5': {
        'bike': 77.98941068926526,
        'ped': 21.418585684227917,
        'residential': 99.40799637349315,
        'school': 0,
        'total': 99.40799637349315,
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
        'bike': 111.02581525327409,
        'ped': 37.7796641021012,
        'residential': 74.3207588837639,
        'school': 66.26980809963025,
        'total': 148.8054793553753,
        'work': 8.21491237198115
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
