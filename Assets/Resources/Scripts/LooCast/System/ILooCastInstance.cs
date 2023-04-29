namespace LooCast.System
{
    using LooCast.System.Data;
    
    public interface ILooCastInstance : ILooCastObject
    {
        #region Properties
        public IInstanceData Data { get; set; }
        #endregion
    }
}
