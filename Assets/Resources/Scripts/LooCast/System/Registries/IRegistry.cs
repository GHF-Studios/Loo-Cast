namespace LooCast.System.Registries
{
    using LooCast.System.Identifiers;

    public interface IRegistry : ILooCastType
    {
        #region Properties
#nullable enable
        public IRegistry? BaseRegistry { get; }
#nullable disable
        #endregion

        #region Methods
        public void Add(IIdentifier key, ILooCastInstance value);
        public bool Remove(IIdentifier key);
        public void Clear();
        #endregion
    }
}
