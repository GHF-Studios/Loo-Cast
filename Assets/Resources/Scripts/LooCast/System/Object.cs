using System.Collections.Generic;
using System;

namespace LooCast.System
{
    using LooCast.System.Paths;

    public class Object : IObject
    {
        #region Properties
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
        protected List<IObject> objectChildrenList;
        #endregion

        #region Constructors
        public Object(string objectName, IFile fileParent)
        {
            if (fileParent == null)
            {
                throw new ArgumentException("Parent File may not be null here, as this would imply the existence of the Parent Object, but the opposite is implied by the choice of this costructor, instead of the constructor, which sets the Parent Object!");
            }
            
            PathBuilder objectPathBuilder = PathBuilder.Load(fileParent.FilePath);
            objectPathBuilder.WithObject(objectName);

            ObjectName = objectName;
            ObjectPath = objectPathBuilder.ConstructObjectPath();
            FileParent = fileParent;
            ObjectParent = null;
            objectChildrenList = new List<IObject>();
        }
        
        public Object(string objectName, IObject objectParent)
        {
            if (objectParent == null)
            {
                throw new ArgumentException("Parent Object may not be null here, as this would imply the existence of the Parent File, but the opposite is implied by the choice of this costructor, instead of the constructor, which sets the Parent File!");
            }

            PathBuilder objectPathBuilder = PathBuilder.Load(objectParent.ObjectPath);
            objectPathBuilder.WithObject(objectName);

            ObjectName = objectName;
            ObjectPath = objectPathBuilder.ConstructObjectPath();
            ObjectParent = objectParent;
            ObjectParent = null;
            objectChildrenList = new List<IObject>();
        }
        #endregion

        #region Methods
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
            if (obj is Object)
            {
                return Equals((Object)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(Object otherObject)
        {
            return otherObject.ObjectPath == this.ObjectPath;
        }

        public override int GetHashCode()
        {
            return ObjectPath.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(Object object1, Object object2)
        {
            return object1.Equals(object2);
        }

        public static bool operator !=(Object object1, Object object2)
        {
            return !object1.Equals(object2);
        }

        public static implicit operator string(Object _object)
        {
            return _object.ObjectPath;
        }
        #endregion
    }
}
