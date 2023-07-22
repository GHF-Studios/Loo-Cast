using System;

namespace LooCast.System
{
    using LooCast.System.Serialization;
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
                    string assemblyQualifiedEntityTypeName = typeof(SystemManager).AssemblyQualifiedName;
                    instance = Entity.Create<SystemManager>();
                    
                    Entity.MetaData instanceMetaData = new Entity.MetaData
                        (
                            assemblyQualifiedEntityTypeName, 
                            new Guid(), 
                            new IComponent.IMetaData[] 
                            {
                                new FolderComponent.MetaData(typeof(FolderComponent).AssemblyQualifiedName)
                            }
                        );
                    
                    Manager.Data instanceData = new Manager.Data
                        (
                            assemblyQualifiedEntityTypeName,
                            new IComponent.IData[]
                            {
                                new FolderComponent.Data
                                    (
                                        typeof(FolderComponent).AssemblyQualifiedName, 
                                        "SystemManager", 
                                        MainManager.Instance.GetComponent<FolderComponent>().FolderPath
                                    )
                            },
                            "SystemManager",
                            MainManager.Instance
                        );
                    
                    
                    ((ISerializable<Entity.MetaData, Manager.Data>)instance).SetMetaData(instanceMetaData);
                    ((ISerializable<Entity.MetaData, Manager.Data>)instance).SetData(instanceData);
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static SystemManager instance;
        #endregion

        #region Constructors
        public SystemManager() : base()
        {
            RegisterSetupAction(() =>
            {
                AddChildModuleManager(FolderManager.Instance);
                AddChildModuleManager(FileManager.Instance);
                AddChildModuleManager(ObjectManager.Instance);
                AddChildModuleManager(EntityManager.Instance); ;
                AddChildModuleManager(ComponentManager.Instance);
            });
        }
        #endregion
    }
}
