using System.Collections.Generic;

namespace LooCast.System
{
    using global::System;
    using LooCast.System.Paths;

    public class ObjectManager : ModuleManager
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
        public ObjectManager() : base("ObjectManager", SystemManager.Instance)
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

        public IObject GetObject(string stringObjectPath)
        {
            if (!ObjectPath.TryParse(stringObjectPath, out ObjectPath? objectPath))
            {
                return null;
            }
            return GetObject(objectPath!);
        }

        public bool ObjectExists(ObjectPath objectPath)
        {
            return registeredObjects.ContainsKey(objectPath);
        }

        public void CreateObject(ObjectPath objectPath)
        {

        }
        #endregion
    }
}
