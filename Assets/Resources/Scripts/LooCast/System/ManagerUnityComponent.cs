using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System
{
    using LooCast.System.ECS;
    
    public sealed class ManagerUnityComponent : UnityComponent
    {
        #region Properties
        public Manager Manager { get; private set; }
        #endregion
        
        #region Unity Callbacks
        private void Awake()
        {
            gameObject.layer = 31;
            gameObject.tag = "INTERNAL";
            DontDestroyOnLoad(this);
        }
        #endregion
        
        #region Methods
        public void Setup(Manager manager)
        {
            if (Manager is not null)
            {
                throw new InvalidOperationException($"Manager reference has already been initialized!");
            }

            Manager = manager;
        }
        #endregion
    }
}
