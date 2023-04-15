namespace LooCast.System
{
    public interface IRegistry : ILooCastObject
    {
        #region Properties
#nullable enable
        public IRegistry? BaseRegistry { get; }
#nullable disable
        #endregion

        #region Methods
        public void Add(Identifier key, ILooCastObject value);
        public bool Remove(Identifier key);
        public void Clear();
        #endregion
    }
}
