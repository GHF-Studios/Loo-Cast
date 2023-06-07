using LooCast.System.Identifiers;
using System;
using System.Collections.Generic;

namespace LooCast.System.Hierarchies
{
    public abstract class Hierarchy<PathType, ElementType> : IHierarchy
        where PathType : HierarchicalObjectPath
        where ElementType : IHierarchicalElement
    {
        #region Properties
        public IObjectIdentifier ObjectIdentifier => HierarchyIdentifier;
        public IHierarchyIdentifier HierarchyIdentifier => hierarchyIdentifier;

        public HierarchicalObjectPath HierarchicalObjectPath => HierarchyFolderPath;
        public FolderPath HierarchyFolderPath => hierarchyFolderPath;

        public HierarchyElementType HierarchyElementType => HierarchyElementType.Folder;

        IEngineObject IChild<IEngineObject>.Parent => ((IChild<IHierarchy>)this).Parent;
        IHierarchy IChild<IHierarchy>.Parent => HierarchyParent;

        IEnumerable<IEngineObject> IParent<IEngineObject>.Children => ((IParent<IHierarchy>)this).Children;
        IEnumerable<IHierarchy> IParent<IHierarchy>.Children => HierarchyChildren;

        IEnumerable<IHierarchicalElement> IParent<IHierarchicalElement>.Children => HierarchyElementChildren;

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
        private HierarchyIdentifier hierarchyIdentifier;
        private FolderPath hierarchyFolderPath;
        
        public IHierarchy HierarchyParent { get; private set; }
        public List<IHierarchy> HierarchyChildren { get; private set; }
        public List<IHierarchicalElement> HierarchyElementChildren { get; private set; }
        #endregion

        #region Constructors
        public Hierarchy(HierarchyIdentifier hierarchyIdentifier, FolderPath hierarchyFolderPath, IHierarchy hierarchyParent)
        {
            this.hierarchyIdentifier = hierarchyIdentifier;
            this.hierarchyFolderPath = hierarchyFolderPath;
            HierarchyParent = hierarchyParent;
            HierarchyChildren = new List<IHierarchy>();
            HierarchyElementChildren = new List<IHierarchicalElement>();
        }
        #endregion

        #region Methods
        public bool Validate()
        {
            return true;
        }

        public void AddElement(IHierarchicalElement hierarchicalElement) 
        {
            
        }
        public void AddElement(ElementType hierarchicalElement) 
        {
            
        }
        
        public bool RemoveElement(HierarchicalObjectPath elementPath) 
        {
            
        }
        public bool RemoveElement(PathType elementPath) 
        {
            
        }
        
        public IHierarchicalElement GetElement(HierarchicalObjectPath elementPath) 
        {
            
        }
        public ElementType GetElement(PathType elementPath) 
        {
            
        }
        
        public bool TryGetElement(HierarchicalObjectPath elementPath, out IHierarchicalElement hierarchicalElement) 
        {
            
        }
        public bool TryGetElement(PathType elementPath, out ElementType hierarchicalElement) 
        {
            
        }
        
        public bool ContainsPath(HierarchicalObjectPath elementPath) 
        {
            
        }
        public bool ContainsPath(PathType elementPath) 
        {
            
        }
        
        public bool ContainsElement(IHierarchicalElement hierarchicalElement) 
        {
            
        }
        public bool ContainsElement(ElementType hierarchicalElement) 
        {
            
        }
        
        public void Clear() 
        {
            
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
    }
}
