namespace LooCast.Core
{
    using LooCast.System;
    
    public sealed class LooCastCoreManager : CoreModuleManager
    {
        #region Static Properties
        public static LooCastCoreManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new LooCastCoreManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static LooCastCoreManager instance;
        #endregion
        
        #region Constructors
        private LooCastCoreManager() : base("LooCastCoreManager")
        {
        }
        #endregion
    }
}
