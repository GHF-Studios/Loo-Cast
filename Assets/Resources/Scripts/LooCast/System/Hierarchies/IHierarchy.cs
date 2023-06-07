using System.Collections.Generic;

namespace LooCast.System.Hierarchies
{
    using LooCast.System.Identifiers;

    public interface IHierarchy : IEngineObject, IHierarchyElement, IChild<IHierarchy>, IParent<IHierarchy>, IParent<IHierarchyElement>
    {
        #region Properties
        IHierarchyIdentifier HierarchyIdentifier { get; }
        IHierarchy HierarchyParent { get; }
        List<IHierarchy> HierarchyChildren { get; }
        List<IHierarchyElement> HierarchyElementChildren { get; }
        #endregion

        #region Methods
        void Add(IHierarchyElement element);
        bool Remove(HierarchicalObjectPath path);
        IHierarchyElement Get(HierarchicalObjectPath path);
        bool ContainsElement(IHierarchyElement element);
        bool ContainsPath(HierarchicalObjectPath path);
        void Clear();
        #endregion
    }
}
