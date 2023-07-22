using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.ECS
{
    using LooCast.System.Serialization;
    using LooCast.System.ECS;
    using LooCast.Core;
    
    public sealed class ComponentManager : ModuleManager
    {
        #region Static Properties
        public static ComponentManager Instance
        {
            get
            {
                if (instance == null)
                {
                    string assemblyQualifiedEntityTypeName = typeof(ComponentManager).AssemblyQualifiedName;
                    instance = Entity.Create<ComponentManager>();

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
                                        "ComponentManager",
                                        SystemManager.Instance.GetComponent<FolderComponent>().FolderPath
                                    )
                            },
                            "ComponentManager",
                            SystemManager.Instance
                        );


                    ((ISerializable<Entity.MetaData, Manager.Data>)instance).SetMetaData(instanceMetaData);
                    ((ISerializable<Entity.MetaData, Manager.Data>)instance).SetData(instanceData);
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static ComponentManager instance;
        #endregion

        #region Fields
        private Dictionary<Guid, IComponent> registeredEntities;
        #endregion

        #region Constructors
        public ComponentManager() : base()
        {
            RegisterPreSetupAction(() =>
            {
                registeredEntities = new Dictionary<Guid, IComponent>();
            });
        }
        #endregion

        #region Methods
        public void RegisterComponent(IComponent component)
        {
            if (!registeredEntities.ContainsKey(component.ComponentID))
            {
                registeredEntities.Add(component.ComponentID, component);
            }
        }

        public void UnregisterComponent(IComponent component)
        {
            if (registeredEntities.ContainsKey(component.ComponentID))
            {
                registeredEntities.Remove(component.ComponentID);
            }
        }

        public IComponent GetComponent(Guid componentID)
        {
            if (registeredEntities.ContainsKey(componentID))
            {
                return registeredEntities[componentID];
            }
            return null;
        }

        public bool TryGetComponent(Guid componentID, out IComponent component)
        {
            if (!registeredEntities.ContainsKey(componentID))
            {
                component = null;
                return false;
            }
            else
            {
                component = registeredEntities[componentID];
                return true;
            }
        }

        public bool IsComponentRegistered(Guid componentID)
        {
            return registeredEntities.ContainsKey(componentID);
        }
        #endregion
    }
}
