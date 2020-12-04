from typing import Optional, Union


def install(
    genome: str,
    custom: Union[str, bool] = False,
    rsync: bool = False,
    bash: bool = True,
    ftp: bool = True,
    fastaPath: Optional[str] = None,
    transcriptPath: Optional[str] = None,
    exomePath: Optional[str] = None,
) -> None:
    ...
