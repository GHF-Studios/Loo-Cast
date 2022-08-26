using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Station
{
    using Data;
    using Targeting;
    using Weapon;
    using Health;

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
        #endregion

        #region Fields
        [SerializeField] private PlayerStationHealth health;
        [SerializeField] private Targeting targeting;
        #endregion

        #region Unity Callbacks
        private void Start()
        {
            Initialize(Data);
        }
        #endregion
    } 
}
