using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.ECS;
    using LooCast.System.Paths;
    using LooCast.System.Serialization;

    [IncompatibleComponents(typeof(FileComponent), typeof(FolderComponent))]
    public sealed class ObjectComponent : Component, IObject, ISerializable<Component.MetaData, ObjectComponent.Data>
    {
        #region Classes
        new public class Data : Component.Data
        {
            #region Properties
            public string ObjectName { get; set; }
            public bool HasFileParent { get; set; }
            public FilePath? ParentFilePath { get; set; }
            public ObjectPath? ParentObjectPath { get; set; }
            #endregion

            #region Constructors
            public Data(string assemblyQualifiedComponentTypeName, string objectName, FilePath parentFilePath) : base(assemblyQualifiedComponentTypeName)
            {
                ObjectName = objectName;
                HasFileParent = true;
                ParentFilePath = parentFilePath;
                ParentObjectPath = null;
            }

            public Data(string assemblyQualifiedComponentTypeName, string objectName, ObjectPath parentObjectPath) : base(assemblyQualifiedComponentTypeName)
            {
                ObjectName = objectName;
                HasFileParent = false;
                ParentFilePath = null;
                ParentObjectPath = parentObjectPath;
            }
            #endregion
        }
        #endregion

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
        private List<IObject> objectChildrenList;
        #endregion

        #region Constructors
        public ObjectComponent() : base()
        {
            objectChildrenList = new List<IObject>();
            
            RegisterPreInitializationAction(() =>
            {
            });

            RegisterPostTerminationAction(() =>
            {
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
        public bool TryAddChildObject(IObject childObject)
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
        public void AddChildObject(IObject childObject)
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

        public bool TryRemoveChildObject(IObject childObject)
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
        public void RemoveChildObject(IObject childObject)
        {
            if (!IsCreated)
            {
                throw new InvalidOperationException($"Object '{this}' is not created yet!");
            }

            objectChildrenList.Remove(childObject);
        }

        public bool TryGetChildObject(string childObjectName, out IObject childObject)
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
        public IObject GetChildObject(string childObjectName)
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

        public bool ContainsChildObject(IObject childObject)
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
        Component.MetaData ISerializable<Component.MetaData, ObjectComponent.Data>.GetMetaData()
        {
            return ((ISerializable<Component.MetaData, Component.Data>)this).GetMetaData();
        }

        ObjectComponent.Data ISerializable<Component.MetaData, ObjectComponent.Data>.GetData()
        {
            if (!HasData)
            {
                throw new InvalidOperationException($"ObjectComponent '{this}' does not have data!");
            }

            if (FileParent == null)
            {
                return new ObjectComponent.Data(ComponentType.AssemblyQualifiedName, ObjectName, ObjectParent.ObjectPath);
            }
            else
            {
                return new ObjectComponent.Data(ComponentType.AssemblyQualifiedName, ObjectName, FileParent.FilePath);
            }
        }

        void ISerializable<Component.MetaData, ObjectComponent.Data>.SetMetaData(Component.MetaData metaData)
        {
            ((ISerializable<Component.MetaData, Component.Data>)this).SetMetaData(metaData);
        }

        void ISerializable<Component.MetaData, ObjectComponent.Data>.SetData(ObjectComponent.Data data)
        {
            if (data.HasFileParent)
            {
                PathBuilder objectPathBuilder = PathBuilder.Load((ObjectPath)data.ParentObjectPath);
                objectPathBuilder.AsAbsolutePath();
                objectPathBuilder.WithObject(data.ObjectName);

                ObjectName = data.ObjectName;
                ObjectPath = objectPathBuilder.ConstructObjectPath();
                FileParent = null;

                ObjectParent.AddChildObject(this);
            }
            else
            {
                PathBuilder objectPathBuilder = PathBuilder.Load((FilePath)data.ParentFilePath);
                objectPathBuilder.AsAbsolutePath();
                objectPathBuilder = objectPathBuilder.WithObject(data.ObjectName);

                ObjectName = data.ObjectName;
                ObjectPath = objectPathBuilder.ConstructObjectPath();
                ObjectParent = null;

                FileParent.AddChildObject(this);
            }

            ((ISerializable<Component.MetaData, Component.Data>)this).SetData(data);
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
