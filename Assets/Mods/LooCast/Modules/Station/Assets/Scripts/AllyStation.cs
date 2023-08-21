using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.EventSystems;
using UnityEngine.Events;

namespace LooCast.Station
{
    using Data;
    using LooCast.Health;
    using LooCast.Inventory;
    using LooCast.Mission;

    public sealed class AllyStation : Station
    {
        #region Data
        public AllyStationData Data;
        #endregion

        #region Properties
        public AllyStationHealth Health
        {
            get
            {
                return health;
            }
        }
        public AllyStationInventory Inventory
        {
            get
            {
                return inventory;
            }
        }
        public MissionProvider MissionProvider
        {
            get
            {
                return missionProvider;
            }
        }
        #endregion

        #region Fields
        [SerializeField] private AllyStationHealth health;
        [SerializeField] private AllyStationInventory inventory;
        [SerializeField] private MissionProvider missionProvider;
        #endregion

        #region Unity Callbacks
        private void Start()
        {
            Initialize(Data);
        }
        #endregion
    } 
}
