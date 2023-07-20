using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.ECS;
    using global::LooCast.System.Paths;

    [IncompatibleComponents(typeof(FileComponent), typeof(FolderComponent))]
    public sealed class ObjectComponent : Component, IObject
    {
        #region Properties
        public bool IsInitialized { get; private set; }
        
        public string ObjectName { get; private set; }

        public IHierarchicalElementPath HierarchicalElementPath => ObjectPath;
        public ObjectPath ObjectPath { get; private set; }

        public HierarchicalElementType HierarchyElementType => HierarchicalElementType.Object;

        IFile IChild<IFile>.Parent => FileParent;
        public IFile FileParent { get; private set; }

        IObject IChild<IObject>.Parent => ObjectParent;
        public IObject ObjectParent { get; private set; }

        IEnumerable<IObject> IParent<IObject>.Children => ObjectChildren;
        public IEnumerable<IObject> ObjectChildren => objectChildrenList;
        #endregion

        #region Fields
        private List<IObject> objectChildrenList;
        #endregion

        #region Constructors
        public ObjectComponent() : base()
        {
            IsInitialized = false;
        }
        #endregion

        #region Methods
        public void Initialize(string objectName, IFile fileParent)
        {
            if (IsInitialized)
            {
                throw new InvalidOperationException("Object has already been initialized!");
            }

            if (fileParent == null)
            {
                throw new ArgumentException("Parent File may not be null here, as this would imply the existence of a Parent Object, but the opposite is implied by the choice of this costructor, instead of the constructor which sets a Parent Object!");
            }

            PathBuilder objectPathBuilder = PathBuilder.Load(fileParent.FilePath);
            objectPathBuilder.AsAbsolutePath();
            objectPathBuilder = objectPathBuilder.WithObject(objectName);

            ObjectName = objectName;
            ObjectPath = objectPathBuilder.ConstructObjectPath();
            FileParent = fileParent;
            ObjectParent = null;
            objectChildrenList = new List<IObject>();

            fileParent.AddChildObject(this);

            ObjectManager.Instance.RegisterObject(this);

            IsInitialized = true;
        }

        public void Initialize(string objectName, IObject objectParent)
        {
            if (IsInitialized)
            {
                throw new InvalidOperationException("Object has already been initialized!");
            }

            if (objectParent == null)
            {
                throw new ArgumentException("Parent Object may not be null here, as this would imply the existence of a Parent File, but the opposite is implied by the choice of this costructor, instead of the constructor which sets a Parent File!");
            }

            PathBuilder objectPathBuilder = PathBuilder.Load(objectParent.ObjectPath);
            objectPathBuilder.AsAbsolutePath();
            objectPathBuilder.WithObject(objectName);

            ObjectName = objectName;
            ObjectPath = objectPathBuilder.ConstructObjectPath();
            ObjectParent = objectParent;
            ObjectParent = null;
            objectChildrenList = new List<IObject>();

            objectParent.AddChildObject(this);
            
            ObjectManager.Instance.RegisterObject(this);

            IsInitialized = true;
        }

        public bool Validate()
        {
            return true;
        }

        public bool TryAddChildObject(IObject childObject)
        {
            if (ContainsChildObject(childObject.ObjectName))
            {
                return false;
            }
            else
            {
                AddChildObject(childObject);
                return true;
            }
        }
        public void AddChildObject(IObject childObject)
        {
            if (ContainsChildObject(childObject))
            {
                throw new InvalidOperationException($"Object '{this}' already contains an Object '{childObject}'!");
            }
            objectChildrenList.Add(childObject);
        }

        public bool TryRemoveChildObject(IObject childObject)
        {
            if (!ContainsChildObject(childObject))
            {
                return false;
            }
            else
            {
                RemoveChildObject(childObject);
                return true;
            }
        }
        public void RemoveChildObject(IObject childObject)
        {
            objectChildrenList.Remove(childObject);
        }

        public bool TryGetChildObject(string childObjectName, out IObject childObject)
        {
            if (!ContainsChildObject(childObjectName))
            {
                childObject = null;
                return false;
            }
            else
            {
                childObject = GetChildObject(childObjectName);
                return true;
            }
        }
        public IObject GetChildObject(string childObjectName)
        {
            return objectChildrenList.Find((objectChild) => { return objectChild.ObjectName == childObjectName; });
        }

        public bool ContainsChildObject(string childObjectName)
        {
            return objectChildrenList.Exists((objectChild) => { return objectChild.ObjectName == childObjectName; });
        }

        public bool ContainsChildObject(IObject childObject)
        {
            return objectChildrenList.Contains(childObject);
        }

        public void ClearChildObjects()
        {
            objectChildrenList.Clear();
        }
        #endregion

        #region Overrides
        public override string ToString()
        {
            return ObjectPath;
        }

        public override bool Equals(object obj)
        {
            if (obj is ObjectComponent)
            {
                return Equals((ObjectComponent)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(ObjectComponent otherObject)
        {
            return otherObject.ObjectPath == this.ObjectPath;
        }

        public override int GetHashCode()
        {
            return ObjectPath.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(ObjectComponent object1, ObjectComponent object2)
        {
            if ((object1 is null && object2 is not null) || (object1 is not null && object2 is null))
            {
                return false;
            }
            else if (object1 is null && object2 is null)
            {
                return true;
            }
            else
            {
                return object1.Equals(object2);
            }
        }

        public static bool operator !=(ObjectComponent object1, ObjectComponent object2)
        {
            if ((object1 is null && object2 is not null) || (object1 is not null && object2 is null))
            {
                return true;
            }
            else if (object1 is null && object2 is null)
            {
                return false;
            }
            else
            {
                return !object1.Equals(object2);
            }
        }

        public static implicit operator string(ObjectComponent _object)
        {
            return _object.ObjectPath;
        }
        #endregion
    }
}
