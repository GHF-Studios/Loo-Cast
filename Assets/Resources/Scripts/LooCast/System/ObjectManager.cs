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
                    string assemblyQualifiedEntityTypeName = typeof(ObjectManager).AssemblyQualifiedName;
                    instance = Entity.Create<ObjectManager>();

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
                                        "ObjectManager",
                                        SystemManager.Instance.GetComponent<FolderComponent>().FolderPath
                                    )
                            },
                            "ObjectManager",
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
            return GetObject(objectPath!);
        }

        public bool TryGetObject(string stringObjectPath, out IObjectComponent _object)
        {
            if (!ObjectPath.TryParse(stringObjectPath, out ObjectPath? objectPath))
            {
                _object = null;
                return false;
            }
            return TryGetObject(objectPath!, out _object);
        }

        public bool IsObjectRegistered(ObjectPath objectPath)
        {
            return registeredObjects.ContainsKey(objectPath);
        }
        #endregion
    }
}
