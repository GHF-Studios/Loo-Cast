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
        public PlayerStationHealth Health
        {
            get
            {
                return health;
            }
        }
        public PlayerStationInventory PlayerStationInventory
        {
            get
            {
                return playerStationInventory;
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
        [SerializeField] private PlayerStationHealth health;
        [SerializeField] private PlayerStationInventory playerStationInventory;
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
