using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Inventory
{
    using Data;
    using Data.Runtime;

    public sealed class PlayerInventory : MonoBehaviour
    {
        #region Data
        public PlayerInventoryData Data;
        public PlayerInventoryRuntimeData RuntimeData;
        #endregion

        private void Start()
        {
            RuntimeData.Initialize(Data);
        }
    }
}