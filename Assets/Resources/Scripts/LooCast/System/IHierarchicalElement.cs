namespace LooCast.System
{
    public interface IHierarchicalElement
    {
        #region Properties
        IHierarchicalElementPath HierarchicalElementPath { get; }
        HierarchicalElementType HierarchyElementType { get; }
        #endregion

        #region Methods
        public bool Validate();
        #endregion
    }
}
