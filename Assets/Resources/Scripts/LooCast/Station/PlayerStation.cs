using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Station
{
    using Data;
    using LooCast.Targeting;
    using LooCast.Health;
    using LooCast.Inventory;
    using LooCast.Mission;

    public sealed class PlayerStation : Station
    {
        #region Data
        public PlayerStationData Data;
        #endregion

        #region Properties
        public PlayerStationHealth Health
        {
            get
            {
                return health;
            }
        }
        public Targeting Targeting
        {
            get
            {
                return targeting;
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
        [SerializeField] private Targeting targeting;
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
