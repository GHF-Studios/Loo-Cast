using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.ECS
{
    public sealed class ComponentManager : ModuleManager
    {
        #region Static Properties
        public static ComponentManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<ComponentManager, Entity.MetaData, Manager.Data>();
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
            registeredEntities = new Dictionary<Guid, IComponent>();

            FolderComponent folderComponent = AddComponent<FolderComponent, Component.MetaData, FolderComponent.Data>();

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedComponentManagerEntityTypeName = typeof(ComponentManager).AssemblyQualifiedName;
                string assemblyQualifiedComponentManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedComponentManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData componentManagerMetaData = new Entity.MetaData();
                componentManagerMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedComponentManagerEntityTypeName;
                componentManagerMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedComponentManagerEntityMetaDataTypeName;
                componentManagerMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedComponentManagerEntityDataTypeName;
                componentManagerMetaData.EntityID = new Guid();

                Manager.Data componentManagerData = new Manager.Data();
                componentManagerData.AssemblyQualifiedEntityTypeName = assemblyQualifiedComponentManagerEntityTypeName;
                componentManagerData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedComponentManagerEntityMetaDataTypeName;
                componentManagerData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedComponentManagerEntityDataTypeName;
                componentManagerData.ManagerName = "ComponentManager";
                componentManagerData.ManagerParent = SystemManager.Instance;

                SetEntityMetaData(componentManagerMetaData);
                SetEntityData(componentManagerData);

                foreach (ISubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPreSetup();
                }

                EntityManager.Instance.RegisterEntity(this);
            });

            RegisterSetupAction(() =>
            {
                string assemblyQualifiedFolderComponentTypeName = typeof(FolderComponent).AssemblyQualifiedName;
                string assemblyQualifiedFolderComponentMetaDataTypeName = typeof(Component.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedFolderComponentDataTypeName = typeof(FolderComponent.Data).AssemblyQualifiedName;

                Component.MetaData folderComponentMetaData = new Component.MetaData();
                folderComponentMetaData.AssemblyQualifiedComponentTypeName = assemblyQualifiedFolderComponentTypeName;
                folderComponentMetaData.AssemblyQualifiedComponentMetaDataTypeName = assemblyQualifiedFolderComponentMetaDataTypeName;
                folderComponentMetaData.AssemblyQualifiedComponentDataTypeName = assemblyQualifiedFolderComponentDataTypeName;
                folderComponentMetaData.ComponentID = new Guid();

                FolderComponent.Data folderComponentData = new FolderComponent.Data();
                folderComponentData.AssemblyQualifiedComponentTypeName = assemblyQualifiedFolderComponentTypeName;
                folderComponentData.AssemblyQualifiedComponentMetaDataTypeName = assemblyQualifiedFolderComponentMetaDataTypeName;
                folderComponentData.AssemblyQualifiedComponentDataTypeName = assemblyQualifiedFolderComponentDataTypeName;
                folderComponentData.FolderName = "ComponentManager";
                folderComponentData.ParentFolderPath = SystemManager.Instance.GetComponent<FolderComponent>().FolderPath;

                folderComponent.SetComponentMetaData(folderComponentMetaData);
                folderComponent.SetComponentData(folderComponentData);

                FolderManager.Instance.RegisterFolder(folderComponent);

                foreach (ISubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnSetup();
                }
            });

            RegisterPostSetupAction(() =>
            {
                foreach (ISubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPostSetup();
                }
            });

            RegisterPreInitializationAction(() =>
            {
                folderComponent.OnPreInitialize();
            });

            RegisterInitializationAction(() =>
            {
                folderComponent.OnInitialize();
            });

            RegisterPostInitializationAction(() =>
            {
                folderComponent.OnPostInitialize();
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
