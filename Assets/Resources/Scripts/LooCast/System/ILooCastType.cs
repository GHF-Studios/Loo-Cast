namespace LooCast.System
{
    using LooCast.System.MetaData;

    public interface ILooCastType : ILooCastObject
    {
        #region Properties
        public IMetaData MetaData { get; set; }
        #endregion
    }
}
