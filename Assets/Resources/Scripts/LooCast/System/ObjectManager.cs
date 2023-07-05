using System;
using System.Collections.Generic;
using System.Linq;

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

        public bool ObjectExists(ObjectPath objectPath)
        {
            return registeredObjects.ContainsKey(objectPath);
        }

        public IObject CreateObject(ObjectPath objectPath)
        {
            if (objectPath == null)
            {
                throw new ArgumentNullException(nameof(objectPath));
            }

            if (ObjectExists(objectPath))
            {
                return null;
            }

            if (objectPath.ObjectPathParent == null && objectPath.FilePathParent == null)
            {
                throw new ArgumentException("ObjectPath must have a parent!", nameof(objectPath));
            }
            else if (objectPath.ObjectPathParent != null && objectPath.FilePathParent != null)
            {
                throw new ArgumentException("ObjectPath cannot have both a parent ObjectPath and a parent FilePath!", nameof(objectPath));
            }
            else if (objectPath.ObjectPathParent != null && objectPath.FilePathParent == null)
            {
                if (!TryGetObject(objectPath.ObjectPathParent, out IObject parentObject))
                {
                    CreateObject(objectPath.ObjectPathParent);
                    parentObject = GetObject(objectPath.ObjectPathParent);
                }

                IObject _object = new Object(objectPath.ObjectName, parentObject);
                RegisterObject(_object);
                return _object;
            }
            else
            {
                if (!FileManager.Instance.TryGetFile(objectPath.FilePathParent, out IFile parentFile))
                {
                    FileManager.Instance.CreateFile(objectPath.FilePathParent);
                    parentFile = FileManager.Instance.GetFile(objectPath.FilePathParent);
                }

                IObject _object = new Object(objectPath.ObjectName, parentFile);
                RegisterObject(_object);
                return _object;
            }
        }

        public void DeleteObject(IObject _object, bool recursive = false)
        {
            if (_object == null)
            {
                throw new ArgumentNullException(nameof(_object));
            }

            if (!ObjectExists(_object.ObjectPath))
            {
                return;
            }

            if (recursive)
            {
                foreach (IObject childObject in ((IParent<IObject>)_object).Children)
                {
                    DeleteObject(childObject, true);
                }
            }
            else
            {
                if (_object.Children.Count() != 0)
                {
                    throw new InvalidOperationException("Object is not empty!");
                }
                else
                {
                    UnregisterObject(_object);
                }
            }
        }
        #endregion
    }
}
