using System;
using System.Collections.Generic;

namespace LooCast.Universe
{
    using LooCast.System;
    using LooCast.Core;
    
    public sealed class UniverseManager : ModuleManager
    {
        #region Static Properties
        public static UniverseManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new UniverseManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static UniverseManager instance;
        #endregion

        #region Fields
        #endregion

        #region Constructors
        private UniverseManager() : base("UniverseManager", LooCastCoreManager.Instance)
        {
            
        }
        #endregion
    }
}
