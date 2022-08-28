using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Station
{
    using Data;
    using LooCast.Targeting;
    using LooCast.Health;
    using LooCast.Inventory;

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
        #endregion

        #region Fields
        [SerializeField] private PlayerStationHealth health;
        [SerializeField] private Targeting targeting;
        [SerializeField] private PlayerStationInventory playerStationInventory;
        #endregion

        #region Unity Callbacks
        private void Start()
        {
            Initialize(Data);
        }
        #endregion
    } 
}
