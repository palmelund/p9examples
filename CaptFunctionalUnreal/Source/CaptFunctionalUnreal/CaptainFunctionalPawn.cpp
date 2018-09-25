// Fill out your copyright notice in the Description page of Project Settings.

#include "CaptainFunctionalPawn.h"


// Sets default values
ACaptainFunctionalPawn::ACaptainFunctionalPawn()
{
 	// Set this pawn to call Tick() every frame.  You can turn this off to improve performance if you don't need it.
	PrimaryActorTick.bCanEverTick = true;

}

// Called when the game starts or when spawned
void ACaptainFunctionalPawn::BeginPlay()
{
	Super::BeginPlay();

	auto viewportSize = FVector2D(GEngine->GameViewport->Viewport->GetSizeXY());
	auto viewportCenter = FVector2D(viewportSize.X / 2, viewportSize.Y / 2);

	vector_min = FVector(0.0f, -(viewportCenter.X / 2.0f) + min_x_buffer, -(viewportCenter.Y / 2.0f) + min_y_buffer);
	vector_max = FVector(0.0f, (viewportCenter.X / 2.0f) - max_x_buffer, (viewportCenter.Y / 2.0f) - max_y_buffer);
	
}

// Called every frame
void ACaptainFunctionalPawn::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);
	
	auto loc = PlayerPaperSprite->GetActorLocation();

	if (move_up_pressed) {
		loc.Z += speed * DeltaTime;
	}

	if (move_down_pressed) {
		loc.Z -= speed * DeltaTime;
	}

	if (move_right_pressed) {
		loc.Y += speed * DeltaTime;
	}

	if (move_left_pressed) {
		loc.Y -= speed * DeltaTime;
	}

	loc = ClampVector(loc, vector_min, vector_max);

	// const FVector()

	// ClampVector(loc, );

	PlayerPaperSprite->SetActorLocation(loc);
}

// Called to bind functionality to input
void ACaptainFunctionalPawn::SetupPlayerInputComponent(UInputComponent* PlayerInputComponent)
{
	Super::SetupPlayerInputComponent(PlayerInputComponent);

	FInputActionBinding move_up_pressed_binding{ "MoveUp", IE_Pressed };
	move_up_pressed_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_up_pressed = true; });
	PlayerInputComponent->AddActionBinding(move_up_pressed_binding);

	FInputActionBinding move_up_released_binding{ "MoveUp", IE_Released };
	move_up_released_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_up_pressed = false; });
	PlayerInputComponent->AddActionBinding(move_up_released_binding);

	FInputActionBinding move_down_pressed_binding{ "MoveDown", IE_Pressed };
	move_down_pressed_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_down_pressed = true; });
	PlayerInputComponent->AddActionBinding(move_down_pressed_binding);

	FInputActionBinding move_down_released_binding{ "MoveDown", IE_Released };
	move_down_released_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_down_pressed = false; });
	PlayerInputComponent->AddActionBinding(move_down_released_binding);

	FInputActionBinding move_right_pressed_binding{ "MoveRight", IE_Pressed };
	move_right_pressed_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_right_pressed = true; });
	PlayerInputComponent->AddActionBinding(move_right_pressed_binding);

	FInputActionBinding move_right_released_binding{ "MoveRight", IE_Released };
	move_right_released_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_right_pressed = false; });
	PlayerInputComponent->AddActionBinding(move_right_released_binding);

	FInputActionBinding move_left_pressed_binding{ "MoveLeft", IE_Pressed };
	move_left_pressed_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_left_pressed = true; });
	PlayerInputComponent->AddActionBinding(move_left_pressed_binding);

	FInputActionBinding move_left_released_binding{ "MoveLeft", IE_Released };
	move_left_released_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this]() {move_left_pressed = false; });
	PlayerInputComponent->AddActionBinding(move_left_released_binding);

	FInputActionBinding reset_position_binding{ "Reset", IE_Pressed };
	reset_position_binding.ActionDelegate.GetDelegateForManualSet().BindLambda([this](){
		auto loc = PlayerPaperSprite->GetActorLocation();
		loc.X = 240;
		loc.Y = -140;
		loc.Z = 120;
		PlayerPaperSprite->SetActorLocation(loc);
	});
	PlayerInputComponent->AddActionBinding(reset_position_binding);
}

int ACaptainFunctionalPawn::GetHealth()
{
	return health;
}

void ACaptainFunctionalPawn::SetHealth(int hp)
{
	health = hp;
	if (health <= 0) {
		// TODO: Death
	}
}
