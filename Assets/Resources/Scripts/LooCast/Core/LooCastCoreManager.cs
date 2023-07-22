using System;

namespace LooCast.Core
{
    using LooCast.System;
    using LooCast.Universe;
    using LooCast.System.Serialization;
    using LooCast.System.ECS;

    public sealed class LooCastCoreManager : CoreModuleManager
    {
        #region Static Properties
        public static LooCastCoreManager Instance
        {
            get
            {
                if (instance == null)
                {
                    string assemblyQualifiedEntityTypeName = typeof(LooCastCoreManager).AssemblyQualifiedName;
                    instance = Entity.Create<LooCastCoreManager>();

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
                                        "LooCastCoreManager",
                                        MainManager.Instance.GetComponent<FolderComponent>().FolderPath
                                    )
                            },
                            "LooCastCoreManager",
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
        private static LooCastCoreManager instance;
        #endregion

        #region Properties
        public Universe Universe { get; private set; }
        public UniverseObserver UniverseObserver { get; private set; }
        #endregion

        #region Constructors
        public LooCastCoreManager() : base()
        {
            RegisterSetupAction(() =>
            {
                AddChildModuleManager(UniverseManager.Instance);
            });

            RegisterInitializationAction(() =>
            {
                Universe = new Universe(32);
                UniverseObserver = new UniverseObserver(256);
            });
        }
        #endregion
    }
}
