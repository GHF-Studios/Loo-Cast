using System;
using UnityEngine;

namespace LooCast.Data
{
    public class ResourceManager
    {
        #region Static Properties
        public static ResourceManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new ResourceManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static ResourceManager instance;
        #endregion

        #region Methods
        public void Initialize()
        {
            Debug.Log($"[ResourceManager] Starting Initialization.");
            Debug.Log($"[ResourceManager] Finished Initialization.");
        }
        #endregion
    }
}
