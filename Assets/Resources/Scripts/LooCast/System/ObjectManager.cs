using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.System
{
    using LooCast.System.Paths;
    using LooCast.System.ECS;

    public sealed class ObjectManager : ModuleManager
    {
        #region Static Properties
        public static ObjectManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<ObjectManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static ObjectManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        private Dictionary<ObjectPath, IObjectComponent> registeredObjects;
        #endregion

        #region Constructors
        public ObjectManager() : base()
        {
            registeredObjects = new Dictionary<ObjectPath, IObjectComponent>();

            FolderComponent folderComponent = AddComponent<FolderComponent, Component.MetaData, FolderComponent.Data>();

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedObjectManagerEntityTypeName = typeof(ObjectManager).AssemblyQualifiedName;
                string assemblyQualifiedObjectManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedObjectManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData objectManagerMetaData = new Entity.MetaData();
                objectManagerMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedObjectManagerEntityTypeName;
                objectManagerMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedObjectManagerEntityMetaDataTypeName;
                objectManagerMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedObjectManagerEntityDataTypeName;
                objectManagerMetaData.EntityID = new Guid();

                Manager.Data objectManagerData = new Manager.Data();
                objectManagerData.AssemblyQualifiedEntityTypeName = assemblyQualifiedObjectManagerEntityTypeName;
                objectManagerData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedObjectManagerEntityMetaDataTypeName;
                objectManagerData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedObjectManagerEntityDataTypeName;
                objectManagerData.ManagerName = "ObjectManager";
                objectManagerData.ManagerParent = SystemManager.Instance;

                SetEntityMetaData(objectManagerMetaData);
                SetEntityData(objectManagerData);

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
                folderComponentData.FolderName = "ObjectManager";
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
        public void RegisterObject(IObjectComponent _object)
        {
            if (!registeredObjects.ContainsKey(_object.ObjectPath))
            {
                registeredObjects.Add(_object.ObjectPath, _object);
            }
        }

        public void UnregisterObject(IObjectComponent _object)
        {
            if (registeredObjects.ContainsKey(_object.ObjectPath))
            {
                registeredObjects.Remove(_object.ObjectPath);
            }
        }

        public IObjectComponent GetObject(ObjectPath objectPath)
        {
            if (registeredObjects.ContainsKey(objectPath))
            {
                return registeredObjects[objectPath];
            }
            return null;
        }

        public bool TryGetObject(ObjectPath objectPath, out IObjectComponent _object)
        {
            if (!registeredObjects.ContainsKey(objectPath))
            {
                _object = null;
                return false;
            }
            else
            {
                _object = registeredObjects[objectPath];
                return true;
            }
        }

        public IObjectComponent GetObject(string objectGUSP)
        {
            if (!ObjectPath.TryParse(objectGUSP, out ObjectPath? objectPath))
            {
                return null;
            }
            return GetObject((ObjectPath)objectPath);
        }

        public bool TryGetObject(string stringObjectPath, out IObjectComponent _object)
        {
            if (!ObjectPath.TryParse(stringObjectPath, out ObjectPath? objectPath))
            {
                _object = null;
                return false;
            }
            return TryGetObject((ObjectPath)objectPath, out _object);
        }

        public bool IsObjectRegistered(ObjectPath objectPath)
        {
            return registeredObjects.ContainsKey(objectPath);
        }
        #endregion
    }
}
