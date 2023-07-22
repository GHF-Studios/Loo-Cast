using System;
using System.Collections.Generic;

namespace LooCast.Universe
{
    using LooCast.System;
    using LooCast.Core;
    using LooCast.System.Serialization;
    using LooCast.System.ECS;

    public sealed class UniverseManager : ModuleManager
    {
        #region Static Properties
        public static UniverseManager Instance
        {
            get
            {
                if (instance == null)
                {
                    string assemblyQualifiedEntityTypeName = typeof(UniverseManager).AssemblyQualifiedName;
                    instance = Entity.Create<UniverseManager>();

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
                                        "UniverseManager",
                                        LooCastCoreManager.Instance.GetComponent<FolderComponent>().FolderPath
                                    )
                            },
                            "UniverseManager",
                            LooCastCoreManager.Instance
                        );


                    ((ISerializable<Entity.MetaData, Manager.Data>)instance).SetMetaData(instanceMetaData);
                    ((ISerializable<Entity.MetaData, Manager.Data>)instance).SetData(instanceData);
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
        public UniverseManager() : base()
        {
            
        }
        #endregion
    }
}
