using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Station
{
    using Data;
    using Spawner;
    using Health;

    [RequireComponent(typeof(EnemyStationHealth))]
    public sealed class EnemyStation : Station
    {
        public EnemyStationData Data;
        public EnemyStationHealth Health { get; private set; }
        public EnemySpawner Spawner { get; private set; }

        private void Start()
        {
            Initialize(Data);

            Health = GetComponent<EnemyStationHealth>();
            Spawner = GetComponentInChildren<EnemySpawner>();
        }
    } 
}
