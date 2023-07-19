namespace LooCast.Core
{
    using LooCast.System;
    using LooCast.System.Numerics;
    using LooCast.Universe;
    
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
            Universe universe = new Universe();
            
            Scale scale_0 = universe.GetScale(0);
            
            for (int i = -8; i < 8; i++)
            {
                for (int j = -8; j < 8; j++)
                {
                    scale_0.GenerateChunk(new BigVec2Int(i, j));
                }
            }
        }
        #endregion
    }
}
