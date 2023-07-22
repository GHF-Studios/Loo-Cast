using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.ECS;
    using LooCast.System.Paths;
    using LooCast.System.Serialization;

    [IncompatibleComponents(typeof(FileComponent), typeof(FolderComponent))]
    public sealed class ObjectComponent : Component, IObjectComponent
    {
        #region Classes
        new public class Data : Component.Data, IObjectComponent.IData
        {
            #region Properties
            public string ObjectName { get; set; }
            public bool HasFileParent { get; set; }
            public FilePath? ParentFilePath { get; set; }
            public ObjectPath? ParentObjectPath { get; set; }
            #endregion
        }
        #endregion

        #region Properties
        public string ObjectName { get; private set; }

        public IHierarchicalElementPath HierarchicalElementPath => ObjectPath;
        public ObjectPath ObjectPath { get; private set; }

        public HierarchicalElementType HierarchyElementType => HierarchicalElementType.Object;

        IFileComponent IChild<IFileComponent>.Parent => FileParent;
        public IFileComponent FileParent { get; private set; }

        IObjectComponent IChild<IObjectComponent>.Parent => ObjectParent;
        public IObjectComponent ObjectParent { get; private set; }

        IEnumerable<IObjectComponent> IParent<IObjectComponent>.Children => ObjectChildren;
        public IEnumerable<IObjectComponent> ObjectChildren => objectChildrenList;
        #endregion

        #region Fields
        private List<IObjectComponent> objectChildrenList;
        #endregion

        #region Constructors
        public ObjectComponent() : base()
        {
            objectChildrenList = new List<IObjectComponent>();
            
            RegisterPreInitializationAction(() =>
            {
                ObjectManager.Instance.RegisterObject(this);
            });

            RegisterPostTerminationAction(() =>
            {
                ObjectManager.Instance.UnregisterObject(this);
                ObjectName = null;
                objectChildrenList = null;
            });
        }
        #endregion

        #region Methods
        public bool Validate()
        {
            return true;
        }

        #region Child Management
        public bool TryAddChildObject(IObjectComponent childObject)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Object '{this}' is not created yet!");
            }

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
        public void AddChildObject(IObjectComponent childObject)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Object '{this}' is not created yet!");
            }

            if (ContainsChildObject(childObject))
            {
                throw new InvalidOperationException($"Object '{this}' already contains an Object '{childObject}'!");
            }
            objectChildrenList.Add(childObject);
        }

        public bool TryRemoveChildObject(IObjectComponent childObject)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Object '{this}' is not created yet!");
            }

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
        public void RemoveChildObject(IObjectComponent childObject)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Object '{this}' is not created yet!");
            }

            objectChildrenList.Remove(childObject);
        }

        public bool TryGetChildObject(string childObjectName, out IObjectComponent childObject)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Object '{this}' is not created yet!");
            }

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
        public IObjectComponent GetChildObject(string childObjectName)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Object '{this}' is not created yet!");
            }

            return objectChildrenList.Find((objectChild) => { return objectChild.ObjectName == childObjectName; });
        }

        public bool ContainsChildObject(string childObjectName)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Object '{this}' is not created yet!");
            }

            return objectChildrenList.Exists((objectChild) => { return objectChild.ObjectName == childObjectName; });
        }

        public bool ContainsChildObject(IObjectComponent childObject)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Object '{this}' is not created yet!");
            }

            return objectChildrenList.Contains(childObject);
        }

        public void ClearChildObjects()
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Object '{this}' is not created yet!");
            }

            objectChildrenList.Clear();
        }
        #endregion

        #region Data Management
        public override IData GetData()
        {
            IObjectComponent.IData objectComponentData = (IObjectComponent.IData)base.GetData();

            if (FileParent == null)
            {
                objectComponentData.ObjectName = ObjectName;
                objectComponentData.HasFileParent = false;
                objectComponentData.ParentObjectPath = ObjectParent.ObjectPath;
                objectComponentData.ParentFilePath = null;
            }
            else
            {
                objectComponentData.ObjectName = ObjectName;
                objectComponentData.HasFileParent = true;
                objectComponentData.ParentObjectPath = null;
                objectComponentData.ParentFilePath = FileParent.FilePath;
            }

            return objectComponentData;
        }

        public override void SetData(IData data)
        {
            IObjectComponent.IData objectComponentData = (IObjectComponent.IData)data;

            if (objectComponentData.HasFileParent)
            {
                PathBuilder objectPathBuilder = PathBuilder.Load((ObjectPath)objectComponentData.ParentObjectPath);
                objectPathBuilder.AsAbsolutePath();
                objectPathBuilder.WithObject(objectComponentData.ObjectName);
                ObjectPath = objectPathBuilder.ConstructObjectPath();
                ObjectName = objectComponentData.ObjectName;
                ObjectParent = ObjectManager.Instance.GetObject(objectComponentData.ParentObjectPath);
                FileParent = null;
                ObjectParent.AddChildObject(this);
            }
            else
            {
                PathBuilder objectPathBuilder = PathBuilder.Load((FilePath)objectComponentData.ParentFilePath);
                objectPathBuilder.AsAbsolutePath();
                objectPathBuilder.WithObject(objectComponentData.ObjectName);
                ObjectPath = objectPathBuilder.ConstructObjectPath();
                ObjectName = objectComponentData.ObjectName;
                ObjectParent = null;
                FileParent = FileManager.Instance.GetFile(objectComponentData.ParentObjectPath);
                FileParent.AddChildObject(this);
            }

            base.SetData(data);
        }
        #endregion

        #endregion

        #region Overrides
        public override string ToString()
        {
            return $"ObjectComponent[{ObjectPath}]";
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
