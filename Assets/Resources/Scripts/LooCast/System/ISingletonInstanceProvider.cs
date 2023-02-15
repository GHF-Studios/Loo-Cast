namespace LooCast.System
{
    public interface ISingletonInstanceProvider
    {
        #region Properties
        Instance LooCastInstance { get; }
        #endregion
    }
}