using System.Collections.Generic;

namespace LooCast.System.Hierarchies
{
    using LooCast.System.Identifiers;

    public interface IHierarchy : IEngineObject, IFolder, IChild<IHierarchy>, IParent<IHierarchy>, IParent<IHierarchicalElement>
    {
        #region Properties
        IHierarchyIdentifier HierarchyIdentifier { get; }
        IHierarchy HierarchyParent { get; }
        List<IHierarchy> HierarchyChildren { get; }
        List<IHierarchicalElement> HierarchyElementChildren { get; }
        #endregion

        #region Methods
        void AddElement(IHierarchicalElement hierarchicalElement);
        bool RemoveElement(IHierarchicalElementPath elementPath);
        IHierarchicalElement GetElement(IHierarchicalElementPath elementPath);
        bool TryGetElement(IHierarchicalElementPath elementPath, out IHierarchicalElement hierarchicalElement);
        bool ContainsPath(IHierarchicalElementPath elementPath);
        bool ContainsElement(IHierarchicalElement hierarchicalElement);
        void Clear();
        #endregion
    }

    public interface IHierarchy<PathType, ElementType> : IHierarchy
        where PathType : IHierarchicalElementPath
        where ElementType : IHierarchicalElement
    {
        #region Methods
        void AddElement(ElementType hierarchicalElement);
        bool RemoveElement(PathType elementPath);
        ElementType GetElement(PathType elementPath);
        bool TryGetElement(PathType elementPath, out ElementType hierarchicalElement);
        bool ContainsPath(PathType elementPath);
        bool ContainsElement(ElementType hierarchicalElement);
        #endregion
    }
}
