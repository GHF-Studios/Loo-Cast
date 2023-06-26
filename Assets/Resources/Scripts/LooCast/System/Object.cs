using System.Collections.Generic;
using System;

namespace LooCast.System
{
    using LooCast.System.Paths;

    public abstract class Object : IObject
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
        public IEnumerable<IObject> ObjectChildren => ObjectChildrenList;

        #region Initialization Phase Flags
        public bool IsEarlyPreInitializing { get; private set; }
        public bool IsPreInitializing { get; private set; }
        public bool IsLatePreInitializing { get; private set; }
        public bool IsEarlyPreInitialized { get; private set; }
        public bool IsPreInitialized { get; private set; }
        public bool IsLatePreInitialized { get; private set; }

        public bool IsEarlyInitializing { get; private set; }
        public bool IsInitializing { get; private set; }
        public bool IsLateInitializing { get; private set; }
        public bool IsEarlyInitialized { get; private set; }
        public bool IsInitialized { get; private set; }
        public bool IsLateInitialized { get; private set; }

        public bool IsEarlyPostInitializing { get; private set; }
        public bool IsPostInitializing { get; private set; }
        public bool IsLatePostInitializing { get; private set; }
        public bool IsEarlyPostInitialized { get; private set; }
        public bool IsPostInitialized { get; private set; }
        public bool IsLatePostInitialized { get; private set; }

        public bool IsFullyPreInitialized
        {
            get
            {
                return IsEarlyPreInitialized && IsPreInitialized && IsLatePreInitialized;
            }
        }
        public bool IsFullyInitialized
        {
            get
            {
                return IsEarlyInitialized && IsInitialized && IsLateInitialized;
            }
        }
        public bool IsFullyPostInitialized
        {
            get
            {
                return IsEarlyPostInitialized && IsPostInitialized && IsLatePostInitialized;
            }
        }
        public bool IsCompletelyInitialized
        {
            get
            {
                return IsFullyPreInitialized && IsFullyInitialized && IsPostInitialized;
            }
        }
        #endregion

        #region Termination Phase Flags
        public bool IsEarlyPreTerminating { get; private set; }
        public bool IsPreTerminating { get; private set; }
        public bool IsLatePreTerminating { get; private set; }
        public bool IsEarlyPreTerminated { get; private set; }
        public bool IsPreTerminated { get; private set; }
        public bool IsLatePreTerminated { get; private set; }

        public bool IsEarlyTerminating { get; private set; }
        public bool IsTerminating { get; private set; }
        public bool IsLateTerminating { get; private set; }
        public bool IsEarlyTerminated { get; private set; }
        public bool IsTerminated { get; private set; }
        public bool IsLateTerminated { get; private set; }

        public bool IsEarlyPostTerminating { get; private set; }
        public bool IsPostTerminating { get; private set; }
        public bool IsLatePostTerminating { get; private set; }
        public bool IsEarlyPostTerminated { get; private set; }
        public bool IsPostTerminated { get; private set; }
        public bool IsLatePostTerminated { get; private set; }

        public bool IsFullyPreTerminated
        {
            get
            {
                return IsEarlyPreTerminated && IsPreTerminated && IsLatePreTerminated;
            }
        }
        public bool IsFullyTerminated
        {
            get
            {
                return IsEarlyTerminated && IsTerminated && IsLateTerminated;
            }
        }
        public bool IsFullyPostTerminated
        {
            get
            {
                return IsEarlyPostTerminated && IsPostTerminated && IsLatePostTerminated;
            }
        }
        public bool IsCompletelyTerminated
        {
            get
            {
                return IsFullyPreTerminated && IsFullyTerminated && IsPostTerminated;
            }
        }
        #endregion

        #endregion

        #region Fields
        protected List<IObject> ObjectChildrenList;
        #endregion

        #region Constructors
        public Object(string objectName, IFile parentFile)
        {
            if (parentFile == null)
            {
                throw new ArgumentException("Parent File may not be null here, as this would imply the existence of the Parent Object, but the opposite is implied by the choice of this costructor, instead of the constructor, which sets the Parent Object!");
            }
            
            PathBuilder objectPathBuilder = PathBuilder.Load(parentFile.FilePath);
            objectPathBuilder.WithObject(objectName);

            ObjectName = objectName;
            ObjectPath = objectPathBuilder.ConstructObjectPath();
            FileParent = parentFile;
            ObjectParent = null;
            ObjectChildrenList = new List<IObject>();
        }
        
        public Object(string objectName, IObject parentObject)
        {
            if (parentObject == null)
            {
                throw new ArgumentException("Parent Object may not be null here, as this would imply the existence of the Parent File, but the opposite is implied by the choice of this costructor, instead of the constructor, which sets the Parent File!");
            }

            PathBuilder objectPathBuilder = PathBuilder.Load(parentObject.ObjectPath);
            objectPathBuilder.WithObject(objectName);

            ObjectName = objectName;
            ObjectPath = objectPathBuilder.ConstructObjectPath();
            ObjectParent = parentObject;
            ObjectParent = null;
            ObjectChildrenList = new List<IObject>();
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
            ObjectChildrenList.Add(childObject);
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
            ObjectChildrenList.Remove(childObject);
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
            return ObjectChildrenList.Find((objectChild) => { return objectChild.ObjectName == childObjectName; });
        }

        public bool ContainsChildObject(string childObjectName)
        {
            return ObjectChildrenList.Exists((objectChild) => { return objectChild.ObjectName == childObjectName; });
        }

        public bool ContainsChildObject(IObject childObject)
        {
            return ObjectChildrenList.Contains(childObject);
        }

        public void ClearChildObjects()
        {
            ObjectChildrenList.Clear();
        }

        #region Initialization Phases
        public virtual void EarlyPreInitialize()
        {

        }

        public virtual void PreInitialize()
        {

        }

        public virtual void LatePreInitialize()
        {

        }

        public virtual void EarlyInitialize()
        {

        }

        public virtual void Initialize()
        {

        }

        public virtual void LateInitialize()
        {

        }

        public virtual void EarlyPostInitalize()
        {

        }

        public virtual void PostInitialize()
        {

        }

        public virtual void LatePostInitialize()
        {

        }
        #endregion

        #region Termination Phases
        public virtual void EarlyPreTerminate()
        {

        }

        public virtual void PreTerminate()
        {

        }

        public virtual void LatePreTerminate()
        {

        }

        public virtual void EarlyTerminate()
        {

        }

        public virtual void Terminate()
        {

        }

        public virtual void LateTerminate()
        {

        }

        public virtual void EarlyPostTerminate()
        {

        }

        public virtual void PostTerminate()
        {

        }

        public virtual void LatePostTerminate()
        {

        }
        #endregion

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
