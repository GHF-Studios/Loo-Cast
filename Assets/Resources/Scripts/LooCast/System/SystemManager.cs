﻿namespace LooCast.System
{
    using LooCast.System.ECS;
    
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
        private SystemManager() : base("SystemManager")
        {
            RegisterPreInitializationAction(() =>
            {
                AddChildModuleManager(FolderManager.Instance);
                AddChildModuleManager(FileManager.Instance);
                AddChildModuleManager(ObjectManager.Instance);
                AddChildModuleManager(EntityManager.Instance);;
                AddChildModuleManager(ComponentManager.Instance);
            });
        }
        #endregion
    }
}
