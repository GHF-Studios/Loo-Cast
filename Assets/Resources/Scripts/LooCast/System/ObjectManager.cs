using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.System
{
    using LooCast.System.Paths;

    public sealed class ObjectManager : ModuleManager
    {
        #region Static Properties
        public static ObjectManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new ObjectManager();
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
        private Dictionary<ObjectPath, IObject> registeredObjects;
        #endregion

        #region Constructors
        private ObjectManager() : base("ObjectManager", SystemManager.Instance)
        {
            registeredObjects = new Dictionary<ObjectPath, IObject>();
        }
        #endregion

        #region Methods
        public void RegisterObject(IObject _object)
        {
            if (!registeredObjects.ContainsKey(_object.ObjectPath))
            {
                registeredObjects.Add(_object.ObjectPath, _object);
            }
        }

        public void UnregisterObject(IObject _object)
        {
            if (registeredObjects.ContainsKey(_object.ObjectPath))
            {
                registeredObjects.Remove(_object.ObjectPath);
            }
        }

        public IObject GetObject(ObjectPath objectPath)
        {
            if (registeredObjects.ContainsKey(objectPath))
            {
                return registeredObjects[objectPath];
            }
            return null;
        }

        public bool TryGetObject(ObjectPath objectPath, out IObject _object)
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

        public IObject GetObject(string objectGUSP)
        {
            if (!ObjectPath.TryParse(objectGUSP, out ObjectPath? objectPath))
            {
                return null;
            }
            return GetObject(objectPath!);
        }

        public bool TryGetObject(string stringObjectPath, out IObject _object)
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
