namespace LooCast.System
{
    using LooCast.System.Identifiers;

    public interface IType : IEngineObject, IIdentifiableObject
    {
        #region Properties
        public ITypeIdentifier TypeIdentifier { get; }
        #endregion
    }
}
