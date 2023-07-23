using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.System
{
    using LooCast.System.Paths;
    using LooCast.System.Serialization;
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

                    string assemblyQualifiedMainManagerEntityTypeName = typeof(ObjectManager).AssemblyQualifiedName;
                    string assemblyQualifiedMainManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                    string assemblyQualifiedMainManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                    string assemblyQualifiedFolderComponentTypeName = typeof(FolderComponent).AssemblyQualifiedName;
                    string assemblyQualifiedFolderComponentMetaDataTypeName = typeof(Component.MetaData).AssemblyQualifiedName;
                    string assemblyQualifiedFolderComponentDataTypeName = typeof(FolderComponent.Data).AssemblyQualifiedName;

                    Entity.MetaData instanceMetaData = new Entity.MetaData();
                    instanceMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedMainManagerEntityTypeName;
                    instanceMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedMainManagerEntityMetaDataTypeName;
                    instanceMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedMainManagerEntityDataTypeName;
                    instanceMetaData.GUID = new Guid();
                    IFolderComponent.IMetaData folderComponentMetaData = new FolderComponent.MetaData();
                    folderComponentMetaData.AssemblyQualifiedComponentTypeName = assemblyQualifiedFolderComponentTypeName;
                    folderComponentMetaData.AssemblyQualifiedComponentMetaDataTypeName = assemblyQualifiedFolderComponentMetaDataTypeName;
                    folderComponentMetaData.AssemblyQualifiedComponentDataTypeName = assemblyQualifiedFolderComponentDataTypeName;
                    folderComponentMetaData.GUID = new Guid();
                    instanceMetaData.ComponentMetaDatas = new IComponent.IMetaData[]
                    {
                        folderComponentMetaData
                    };

                    Manager.Data instanceData = new Manager.Data();
                    instanceData.AssemblyQualifiedEntityTypeName = assemblyQualifiedMainManagerEntityTypeName;
                    instanceData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedMainManagerEntityMetaDataTypeName;
                    instanceData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedMainManagerEntityDataTypeName;
                    IFolderComponent.IData folderComponentData = new FolderComponent.Data();
                    folderComponentData.AssemblyQualifiedComponentTypeName = assemblyQualifiedFolderComponentTypeName;
                    folderComponentData.AssemblyQualifiedComponentMetaDataTypeName = assemblyQualifiedFolderComponentMetaDataTypeName;
                    folderComponentData.AssemblyQualifiedComponentDataTypeName = assemblyQualifiedFolderComponentDataTypeName;
                    folderComponentData.FolderName = "ObjectManager";
                    folderComponentData.ParentFolderPath = SystemManager.Instance.GetComponent<FolderComponent>().FolderPath;
                    instanceData.ComponentDatas = new IComponent.IData[]
                    {
                        folderComponentData
                    };
                    instanceData.ManagerName = "ObjectManager";
                    instanceData.ManagerParent = SystemManager.Instance;

                    instance.SetMetaData(instanceMetaData);
                    instance.SetData(instanceData);
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
