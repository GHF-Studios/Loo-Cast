using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Health
{
    using Data;

    public class PlayerStationHealth : StationHealth
    {
        public PlayerStationHealthData Data;

        private void Start()
        {
            Initialize(Data);
        }
    } 
}
