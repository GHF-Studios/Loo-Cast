namespace LooCast.System
{
    public sealed class SystemManager : CoreModuleManager
    {
        #region Static Properties
        public static SystemManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new SystemManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static SystemManager instance;
        #endregion

        #region Constructors
        public SystemManager() : base("SystemManager")
        {
            AddChildModuleManager(new FolderManager());
        }
        #endregion
    }
}
